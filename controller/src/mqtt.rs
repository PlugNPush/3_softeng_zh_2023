use defmt::Format;
use embassy_net::{
    tcp::{ConnectError, Error, TcpSocket},
    IpAddress, Stack,
};
use embassy_net_wiznet::Device;
use embassy_time::Duration;
use mqtt_protocol::{connect::Connect, publish::Publish};

pub const BUFFER_SIZE: usize = 4096;
const TCP_TIMEOUT_SECS: u64 = 5;

#[derive(Debug, Format)]
pub enum MqttError {
    TcpError(Error),
    ConnectError(ConnectError),
}

impl From<ConnectError> for MqttError {
    fn from(err: ConnectError) -> Self {
        MqttError::ConnectError(err)
    }
}

impl From<Error> for MqttError {
    fn from(err: Error) -> Self {
        MqttError::TcpError(err)
    }
}

/// Very simple MQTT client, capable of sending simple publish packets over unencrypted TCP/IP.
pub struct Client<'a> {
    pub client_id: &'a str,
    socket: TcpSocket<'a>,
}

impl<'a> Client<'a> {
    /// Initialize the socket for sending MQTT data and set the socket timeout to `TCP_TIMEOUT_SECS`.
    ///
    /// - `client_id`: Identification of this client.
    /// - `net_stack`: network stack for sending data.
    /// - `rx_buffer`: byte buffer for receiving data.
    /// - `tx_buffer`: byte buffer for sending data.
    pub fn new(
        client_id: &'a str,
        net_stack: &'a Stack<Device<'a>>,
        rx_buffer: &'static mut [u8; BUFFER_SIZE],
        tx_buffer: &'static mut [u8; BUFFER_SIZE],
    ) -> Self {
        let mut socket = embassy_net::tcp::TcpSocket::new(net_stack, rx_buffer, tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(TCP_TIMEOUT_SECS)));

        Self { client_id, socket }
    }

    //// Publish the payload `payload` to the MQTT topic `topic` on the broker running at `address`:1883.
    pub async fn publish(
        &mut self,
        address: IpAddress,
        topic: &str,
        payload: &[u8],
    ) -> Result<(), MqttError> {
        self.socket.connect((address, 1883)).await?;
        let connect = Connect::new(self.client_id);
        self.socket.write(&connect.data[0..connect.length]).await?;

        let publish = Publish::new(topic, payload);
        self.socket.write(&publish.data[0..publish.length]).await?;

        self.socket.close();

        Ok(())
    }
}
