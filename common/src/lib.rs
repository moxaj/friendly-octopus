pub type ClientMessage = String;
pub type ServerMessage = String;

/*
    Game manager:
      sent messages:
        send message to client
        kick client

    On new client connection:
      register client sink to shared state

      clone a new sender
      send 'client connected'
      for each message: send 'received client message'
      send 'client_disconnected'

    On 'send client message'
      send it to each registered sink
*/