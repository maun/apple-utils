use std::cell::Cell;
use std::ops::Deref;
use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{AllocAnyThread, DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSArray, NSObject, NSObjectProtocol, NSString, NSURL};
use objc2_ui_kit::{UIApplication, UIDocumentPickerDelegate, UIDocumentPickerViewController};
use objc2_uniform_type_identifiers::UTType;
use tokio::sync::oneshot;

use super::presentation_style::PresentationStyle;

pub struct FilePicker {
    pub present_animated: bool,
    pub allowed_extensions: Vec<String>,
    pub multiple_selection: bool,
    pub show_file_extensions: bool,
    pub directory_path: Option<PathBuf>,
}

impl Default for FilePicker {
    fn default() -> Self {
        FilePicker {
            present_animated: true,
            allowed_extensions: vec![],
            multiple_selection: true,
            show_file_extensions: false,
            directory_path: None,
        }
    }
}

impl FilePicker {
    pub async fn open(self: &Self) -> Vec<PathBuf> {
        let mtm = MainThreadMarker::new().expect("Must run on main thread");
        let app = UIApplication::sharedApplication(mtm);
        let (result_sender, receiver) = tokio::sync::oneshot::channel::<Vec<PathBuf>>();
        let (_delegate, picker) = self.build_picker(mtm, result_sender);
    
        unsafe {
            let window = app.keyWindow().unwrap();
            let current_vc = window.rootViewController().unwrap();
            current_vc.presentViewController_animated_completion(&picker, self.present_animated, None);
            receiver.await.unwrap()
        }
    }
    
    fn build_picker(
        &self,
        mtm: MainThreadMarker,
        result_sender: oneshot::Sender<Vec<PathBuf>>,
    ) -> (Retained<Delegate>, Retained<UIDocumentPickerViewController>) {
        unsafe {
            let document_types: Vec<_> = self
                .allowed_extensions
                .iter()
                .flat_map(|s| UTType::typeWithFilenameExtension(NSString::from_str(&s).deref()))
                .collect();
            let document_types: Vec<_> = document_types.iter().map(|t| t.deref()).collect();
            let document_types = NSArray::from_slice(document_types.as_slice());

            let picker = UIDocumentPickerViewController::alloc(mtm);
            let picker = UIDocumentPickerViewController::initForOpeningContentTypes_asCopy(
                picker,
                &document_types,
                true,
            );
            let delegate = Delegate::new(mtm, result_sender);
            picker.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
            picker.setAllowsMultipleSelection(self.multiple_selection);
            picker.setShouldShowFileExtensions(self.show_file_extensions);

            if let Some(path) = &self.directory_path {
                let path = path
                    .canonicalize()
                    .expect(&format!("Failed to canonicalize directory path {:?}", path));
                let path = path.to_str().expect("Failed to convert path to string");
                let url = NSURL::alloc();
                let url = NSURL::initFileURLWithPath(url, NSString::from_str(path).deref());
                picker.setDirectoryURL(Some(&url));
            }

            (delegate, picker)
        }
    }
}

struct DelegateIvars {
    result_sender: Cell<Option<oneshot::Sender<Vec<PathBuf>>>>,
}

impl DelegateIvars {
    fn new(result_sender: oneshot::Sender<Vec<PathBuf>>) -> Self {
        Self {
            result_sender: Cell::new(Some(result_sender)),
        }
    }
}

define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[name = "Delegate"]
    #[ivars = DelegateIvars]
    struct Delegate;

    // SAFETY: `NSObjectProtocol` has no safety requirements.
    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl UIDocumentPickerDelegate for Delegate {
        #[unsafe(method(documentPicker:didPickDocumentsAtURLs:))]
        fn document_picker_did_pick_documents_at_urls(
            &self,
            _document_picker: &UIDocumentPickerViewController,
            urls: &NSArray<NSURL>,
        ) {
            let mut url_paths: Vec<PathBuf> = Vec::with_capacity(urls.count());
            for i in 0..urls.count() {
                let url = unsafe { urls.objectAtIndex(i).path().unwrap().to_string() };
                url_paths.push(PathBuf::from(url));
            }
            self.ivars()
                .result_sender
                .take()
                .unwrap()
                .send(url_paths)
                .unwrap();
        }

        #[unsafe(method(documentPickerWasCancelled:))]
        fn document_picker_was_cancelled(&self, _document_picker: &UIDocumentPickerViewController) {
            self.ivars()
                .result_sender
                .take()
                .unwrap()
                .send(Vec::new())
                .unwrap();
        }
    }
);

impl Delegate {
    fn new(mtm: MainThreadMarker, sender: oneshot::Sender<Vec<PathBuf>>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(DelegateIvars::new(sender));
        // SAFETY: The signature of `NSObject`'s `init` method is correct.
        unsafe { msg_send![super(this), init] }
    }
}
