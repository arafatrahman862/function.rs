#![allow(non_snake_case)]

//! ### Keeping track of clients
//!
//! This doesn't directly relate to the WebSocket protocol,
//! but it's worth mentioning here: your server must keep track of clients' sockets so you don't keep handshaking again with clients who have already completed the handshake.
//! The same client IP address can try to connect multiple times.
//! However, the server can deny them if they attempt too many connections in order to save itself from [Denial-of-Service attack](https://en.wikipedia.org/wiki/Denial-of-service_attack).
//!
//!
//! For example, you might keep a table of usernames or ID numbers along with the corresponding WebSocket and other data that you need to associate with that connection.

mod rsv;
mod serde;
pub mod utils;

pub use rsv::Rsv;

/// ### Data frames
///
/// ```txt
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-------+-+-------------+-------------------------------+
/// |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
/// |I|S|S|S|  (4)  |A|     (7)     |             (16/64)           |
/// |N|V|V|V|       |S|             |   (if payload len==126/127)   |
/// | |1|2|3|       |K|             |                               |
/// +-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
/// |     Extended payload length continued, if payload len == 127  |
/// + - - - - - - - - - - - - - - - +-------------------------------+
/// |                               |Masking-key, if MASK set to 1  |
/// +-------------------------------+-------------------------------+
/// | Masking-key (continued)       |          Payload Data         |
/// +-------------------------------- - - - - - - - - - - - - - - - +
/// :                     Payload Data continued ...                :
/// + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
/// |                     Payload Data continued ...                |
/// +---------------------------------------------------------------+
/// ```
pub struct Frame {
    /// Indicates that this is the final fragment in a message.  The first
    /// fragment MAY also be the final fragment.
    pub FIN: bool,

    /// MUST be `false` unless an extension is negotiated that defines meanings
    /// for non-zero values.  If a nonzero value is received and none of
    /// the negotiated extensions defines the meaning of such a nonzero
    /// value, the receiving endpoint MUST _Fail the WebSocket Connection_.
    pub RSV: Rsv,

    pub opcode: Opcode,

    /// Defines whether the "Payload data" is masked.  If set to 1, a
    /// masking key is present in masking-key, and this is used to unmask
    /// the "Payload data" as per Section 5.3.  All frames sent from
    /// client to server have this bit set to 1.
    ///
    /// ### Required for client
    /// A client MUST mask all frames that it sends to the server. (Note
    /// that masking is done whether or not the WebSocket Protocol is running
    /// over TLS.)  The server MUST close the connection upon receiving a
    /// frame that is not masked.
    ///
    /// Note: A server MUST NOT mask any frames that it sends to the client.
    _MASK: (),

    pub payload_len: usize,

    // The masking key is a 32-bit value chosen at random by the client
    pub masking_key: Option<[u8; 4]>,
}

pub enum Opcode {
    /// The FIN and opcode fields work together to send a message split up into separate frames. This is called message fragmentation.
    ///
    /// ```txt
    /// Client: FIN=1, opcode=0x1, msg="hello"
    /// Server: (process complete message immediately) Hi.
    /// Client: FIN=0, opcode=0x1, msg="and a"
    /// Server: (listening, new message containing text started)
    /// Client: FIN=0, opcode=0x0, msg="happy new"
    /// Server: (listening, payload concatenated to previous message)
    /// Client: FIN=1, opcode=0x0, msg="year!"
    /// Server: (process complete message) Happy new year to you too!
    /// ```
    ///
    /// ### Note
    ///
    /// - Control frames MAY be injected in the middle of
    ///   a fragmented message.  Control frames themselves MUST NOT be
    ///   fragmented. An endpoint MUST be capable of handling control frames in the
    ///   middle of a fragmented message.
    ///
    Continuation = 0,

    Text = 1,
    Binary = 2,

    // 3-7 are reserved for further non-control frames.

    // Those are control frames. All control frames MUST have a payload length of 125 bytes or less
    // and MUST NOT be fragmented.
    ///
    /// - The Close frame MAY contain a body that indicates a reason for closing.
    ///
    /// - If there is a body, the first two bytes of the body MUST be a 2-byte unsigned integer (in network byte order: Big Endian)
    ///   representing a status code with value /code/ defined in [Section 7.4](https://datatracker.ietf.org/doc/html/rfc6455#section-7.4). Following the 2-byte integer,
    ///
    /// - Close frames sent from client to server must be masked.
    /// - The application MUST NOT send any more data frames after sending a `Close` frame.
    ///
    /// - If an endpoint receives a Close frame and did not previously send a
    ///   Close frame, the endpoint MUST send a Close frame in response.  (When
    ///   sending a Close frame in response, the endpoint typically echos the
    ///   status code it received.)  It SHOULD do so as soon as practical.  An
    ///   endpoint MAY delay sending a Close frame until its current message is
    ///   sent
    ///
    /// - After both sending and receiving a Close message, an endpoint
    ///   considers the WebSocket connection closed and MUST close the
    ///   underlying TCP connection.
    Close = 8,

    /// A Ping frame MAY include "Application data".
    /// Unless it already received a Close frame.  It SHOULD respond with Pong frame as soon as is practical.
    ///
    /// A Ping frame may serve either as a keepalive or as a means to verify that the remote endpoint is still responsive.
    Ping = 9,

    /// A Pong frame sent in response to a Ping frame must have identical
    /// "Application data" as found in the message body of the Ping frame being replied to.
    ///
    /// If an endpoint receives a Ping frame and has not yet sent Pong frame(s) in response to previous Ping frame(s), the endpoint MAY
    /// elect to send a Pong frame for only the most recently processed Ping frame.
    ///
    ///  A Pong frame MAY be sent unsolicited.  This serves as a unidirectional heartbeat.  A response to an unsolicited Pong frame is not expected.
    Pong = 10,
    // 11-15 are reserved for further control frames
}

pub enum CloseStatusCodes {
    NormalClosure = 1000,
    GoingAway = 1001,
    ProtocolError = 1002,
    UnsupportedData = 1003,
    // reserved 1004
    NoStatusRcvd = 1005,
    AbnormalClosure = 1006,
    InvalidFramePayloadData = 1007,
    PolicyViolation = 1008,
    MessageTooBig = 1009,
    MandatoryExt = 1010,
    InternalError = 1011,
    ServiceRestart = 1012,
    TryAgainLater = 1013,
    TLSHandshake = 1015,
}

#[test]
fn test_name() {
    println!("{:#?}", std::mem::size_of::<Frame>());
    println!("{:#?}", std::mem::size_of::<CloseStatusCodes>());
}
