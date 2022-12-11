import React, { Children, useEffect, useRef, useState } from 'react'
import { useParams } from 'react-router-dom';
import { toast, ToastContainer } from 'react-toastify';
import './assets/styles/VideoChatRoom.scss';
import global from 'global'
import * as process from "process";
global.process = process;

import SimplePeer from 'simple-peer';
import Video from './Video';
const serverURL = import.meta.env.VITE_SERVER_URL;


interface SignalMessage {
  from_user_id: string;
  user_id: string;
  signal: any;
};


const VideoChatRoom = () => {

  const myVideo = useRef<any>(null);
  const peersRef = useRef({});
  const { username } = useParams();
  const userIdRef = useRef<string>();
  const ws = useRef<WebSocket>();
  const [streams, setStreams] = useState<any>({});

  const onSocketMessage = (data: any, stream: MediaStream) => {
    const msgType: string = Object.keys(data.MsgType)[0].toString().toLowerCase();
    const userId = userIdRef.current;
    const peers = peersRef.current;

    switch (msgType) {
      case 'roster':
        if (!userId)
          return;
        const userIds = data.MsgType.Roster as string[];
        const userIdsToAdd = userIds.filter((id: string) => !peers[id] && id != userId);
        userIdsToAdd.forEach((user_id) => {
          createPeer(user_id, true, stream);
        });

        break;

      case 'signal':
        if (!userId)
          return;
        console.log('signal recieved : ', data.MsgType.Signal as SignalMessage);
        const signalMessage = data.MsgType.Signal as SignalMessage;
        signalMessage.signal = JSON.parse(signalMessage.signal);
        if (!peers[signalMessage.from_user_id]) {
          createPeer(signalMessage.from_user_id, false, stream);
        }
        signalPeer(signalMessage);
        break;

      case 'userinfo':
        debugger;
        console.log('got user id', data.MsgType);
        userIdRef.current = data.MsgType.UserInfo;
        break;

      default:
        break;
    }
  }

  const signalPeer = (signal: SignalMessage) => {
    const peer = peersRef.current[signal.from_user_id];
    if (!peer) {
      toast.error('Unable to find the peer to signal');
      return;
    }

    peer.signal(signal.signal);
  }

  // get the user's audio and video and send it to peers. Should be called whenever a new user is added
  useEffect(() => {
    navigator.mediaDevices.getUserMedia({ audio: true, video: { width: 1280, height: 720 } })
      .then((videoStream: MediaStream) => {
        let video = myVideo.current;
        video.srcObject = videoStream;
        video.play();
        if (!ws.current) {
          ws.current = new WebSocket(`${serverURL}/live/${username}`);
        }

        ws.current.onmessage = (message) => {
          onSocketMessage(JSON.parse(message.data), videoStream);
        };
      })
      .catch((err) => {
        console.error(err);
        toast('Unable to load camera or mic');
      })
  }, []);

  useEffect(() => {
    setStreams(Object.entries(peersRef.current).map((entry) => {
      const [peerId, peer] = entry;
      const stream = peer.stream;
      return { [peerId]: stream };
    }));
    console.log('streams', streams);
  }, [peersRef]);

  function createPeer(user_id: string, initiator: boolean, stream: MediaStream): void {
    if (user_id == userIdRef.current) {
      return;
    }

    const peer = new SimplePeer({ initiator: initiator, stream: stream, trickle: false });

    peer.on('signal', (signal: any) => {
      console.log('sending singal', signal, user_id);
      //send signal to this peer
      const signalMessage: SignalMessage = {
        user_id: user_id,
        signal: JSON.stringify(signal),
        from_user_id: userIdRef.current as string
      };

      ws.current?.send(JSON.stringify(signalMessage));
    });

    peer.on('close', (_) => {
      delete peersRef.current[user_id];
      setStreams((oldStreams: any) => {
        let newSteams = { ...oldStreams };
        delete newSteams[user_id];
        return newSteams;
      });
    })

    peer.on('stream', (incomingStream) => {
      setStreams((oldStreams: any) => ({ ...oldStreams, [user_id]: incomingStream }))
    })

    peer.on('error', (error) => {
      console.log('error in peer', error);
    });

    peer.on('connect', () => {
      toast.dismiss();
      toast.success('connected');
    })

    peersRef.current[user_id] = peer;
  }

  return (
    <div>
      <h1>Welcome, {username}</h1>
      <div className='videoGrid'>
        <video className='video' ref={myVideo} autoPlay></video>
        {Object.entries(streams).map((entry, index) => {
          const [peerId, peer] = entry;
          console.log("adding stream : ", peerId);
          return (
            <Video key={index} stream={peer} />
          )
        })}
      </div>
      <ToastContainer
        position="bottom-right"
        theme='dark' />
    </div>
  )
}

export default VideoChatRoom;

function onSocketOpen(event: Event): any {
  toast('You are now connected');
}

function onSocketClose(event: CloseEvent): any {
  toast('You are now disconnected');
}