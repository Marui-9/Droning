use crate::routing::*;
use crate::fragment::*;
pub struct Message{
  //  source_routing_header: SourceRoutingHeader,//source routing might be decided at whatev layer
    //does this make sense ? : fragment_header: ,
    message_header: MessageHeader, //is only source id: Option<u64>
    //m: M, //m
    message_content: MessageContent
}

enum MessageContent{
    ChatMessage(ChatMessage), // chat == communication server
    TextMessage(TextMessage), // text == content server
    MediaMessage(MediaMessage)// media == content server
}
impl Message{
    fn serialize(&self) -> String{unimplemented!()}
    //takes message and returns the data struct serialized in a String
    //so it goes from the actual data struct to a String
    fn deserialize(serialized: String) -> Message{unimplemented!()}
    //Takes the content String and makes an instance of Message from it
    fn disassembly(serialized: String) -> Vec<Fragment>{unimplemented!()}
    //takes the String and splits it into Fragments
    fn assembly(fragments: Vec<Fragment>) -> String{unimplemented!()}
    //takes a bunch of Fragments and composes them in a serialized string.
}
enum ChatMessage{
    ChatRequest(ChatRequest),
    ChatResponse(ChatResponse)
}
enum ChatRequest{ //(chat == communication server)
    ClientList, // => C -> S : client_list?
    MessageFor { // => C -> S : message_for?(client_id, message)
        // note: message_size omitted!
        client: u64,
        message: String,
    },
}
enum ChatResponse{
    ClientList(u64, Vec<u64>), // => S -> C : client_list!(list_length, list_of_client_ids)
    MessageFrom{ // => S -> C : message_from!(client_id, message)
        // note: message_size omitted!
        client: u64,
        message: String
    },
    ErrWrongClient // => S -> C : error_wrong_client_id!
}
enum TextMessage{// (text == media server with text)
    TextRequest(TextRequest), //-> enum
    TextResponse(TextResponse) //-> enum
}
enum TextRequest{

}
enum TextResponse{

}
enum MediaMessage{
    MediaRequest(MediaRequest),
    MediaResponse(MediaResponse)
}
enum MediaRequest{

}
enum MediaResponse{

}
struct MessageHeader {
    /// ID of client or server
    source_id: Option<u64>,
}

