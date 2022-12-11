import React, { useEffect, useRef } from 'react'


const Video = (stream) => {
  const ref = useRef();

  useEffect(() => {
    console.log('stream', stream);
    ref.current.srcObject = stream.stream;
  }, []);

  return <video playsInline autoPlay ref={ref} />
}


export default Video