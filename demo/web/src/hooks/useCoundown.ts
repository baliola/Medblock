import { useEffect, useState } from 'react';

export default function useCountdown() {
  const [secondLeft, setSecondLeft] = useState(0);

  useEffect(() => {
    if (secondLeft <= 0) return;
    const timeout = setTimeout(() => {
      setSecondLeft(secondLeft - 1);
    }, 1000);

    return () => clearTimeout(timeout);
  }, [secondLeft]);

  function start(seconds: number) {
    setSecondLeft(seconds);
  }

  return { secondLeft, start };
}
