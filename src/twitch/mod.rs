use nvim_oxi::{libuv::AsyncHandle, Result};
use tokio::sync::mpsc::UnboundedSender;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

use crate::plugin::config::Config;

#[derive(Debug)]
pub enum TwitchCommand {
    Message(String, String),
    ColorScheme(String),
    VimMotionsHell,
}

#[derive(Debug)]
pub struct TwitchCommandPayload {
    pub command: TwitchCommand,
}

#[tokio::main(flavor = "current_thread")]
pub async fn init(
    handle: AsyncHandle,
    sender: UnboundedSender<TwitchCommandPayload>,
    config: Config,
) -> Result<()> {
    let client_config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            if let ServerMessage::Privmsg(msg) = message {
                let mut split = msg.message_text.trim().splitn(2, ' ');

                let command = split.next();
                let argument = split.next();

                if let Some((command, argument)) = command.zip(argument) {
                    if command == config.commands.message {
                        sender
                            .send(TwitchCommandPayload {
                                command: TwitchCommand::Message(
                                    msg.sender.name,
                                    argument.to_owned(),
                                ),
                            })
                            .unwrap();

                        handle.send().unwrap();
                    }

                    if command == config.commands.colorscheme {
                        sender
                            .send(TwitchCommandPayload {
                                command: TwitchCommand::ColorScheme(argument.to_owned()),
                            })
                            .unwrap();

                        handle.send().unwrap();
                    }

                    if command == config.commands.hell {
                        sender
                            .send(TwitchCommandPayload {
                                command: TwitchCommand::VimMotionsHell,
                            })
                            .unwrap();

                        handle.send().unwrap();
                    }
                }
            }
        }
    });

    if let Some(channel) = config.channel {
        client.join(channel).unwrap_or_else(|e| {
            println!("{e}");
        });
    }

    join_handle.await.unwrap();

    Ok(())
}
