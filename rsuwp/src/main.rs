extern crate winrt;

use winrt::*; // import various helper types
use winrt::windows::foundation::*; // import namespace Windows.System.Diagnostics
use winrt::windows::web::syndication::*;
use winrt::windows::security::credentials::*;
use winrt::windows::data::xml::dom::*;
use winrt::windows::ui::notifications::*;

// fn Sample() {
//     let base = FastHString::new("http://kennykerr.ca");
//     let relative = FastHString::new("feed");
//     let uri = Uri::create_with_relative_uri(&base, &relative).unwrap();
//     let client = SyndicationClient::create_syndication_client();
//     // SyndicationClient client;
//     // SyndicationFeed feed = co_await client.RetrieveFeedAsync(uri);

//     // for (SyndicationItem item : feed.Items()) {
//     //     hstring title = item.Title().Text();
//     //     printf("%ls\n", title.c_str());
//     // }
// }

fn main() {
    toast();
    //init_apartment(ApartmentType::MTA);
    // let infos = ProcessDiagnosticInfo::get_for_processes().unwrap().unwrap();
    // println!("Currently executed processes ({}):", infos.get_size().unwrap());
    // for p in &infos {
    //     let p = p.unwrap();
    //     let pid = p.get_process_id().unwrap();
    //     let exe = p.get_executable_file_name().unwrap();
    //     println!("[{}] {}", pid, exe);
    // }
}


fn toast() {
    // Get a toast XML template
    let toast_xml = ToastNotificationManager::get_template_content(ToastTemplateType::ToastText02).unwrap().unwrap();

    // Fill in the text elements
    let toast_text_elements = toast_xml.get_elements_by_tag_name(&FastHString::new("text")).unwrap().unwrap();
    
    toast_text_elements.item(0).unwrap().unwrap().append_child(&toast_xml.create_text_node(&FastHString::new("Hello from Rust!")).unwrap().unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();
    toast_text_elements.item(1).unwrap().unwrap().append_child(&toast_xml.create_text_node(&FastHString::new("This is some more text.")).unwrap().unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();

    // could use the following to get the XML code for the notification
    println!("{}", toast_xml.query_interface::<IXmlNodeSerializer>().unwrap().get_xml().unwrap());

    // Create the toast and attach event listeners
    let toast = ToastNotification::create_toast_notification(&toast_xml).unwrap();

    // Show the toast. Use PowerShell's App ID to circumvent the need to register one (this is only an example!).
    ToastNotificationManager::create_toast_notifier_with_id(&FastHString::new("{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe")).unwrap().unwrap().show(&toast).unwrap();
}