extern crate notify_send;
use notify_send::Notification;
fn main()
{

    Notification::new()
        .summary("Chromium Crashed")
        .appname("chromium")
        .body(&format!("This is <b>{}</b>!<br/>", "a lie"))
        .icon("chromium")
        .send();


    Notification::new()
        .summary("Table Test - will probably not work")
        .body(&format!("<table>
                       <tr><td>{}</td><td>cell 2</td></tr> <tr><td>cell 3</td><td>cell 4</td></tr>
                       </table>", "cell 1"))
        .icon("table")
        .send();


    Notification::new()
        .summary("Paragraphs")
        .body("This should be a paragraph, this is why I wrote more than one line worth of text, only to make it linebreak.
              This should be another paragraph.")
        .icon("thunderbird")
        .send();

}

