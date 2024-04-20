import { useRouter } from 'next/router';
import React, { useEffect, useState } from 'react';

import BasicButton from '@/components/Button/BasicButton';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const ConsentCodePage = () => {
  const router = useRouter();
  const [showImage, setShowImage] = useState<boolean>(false);
  const { consent } = router.query;

  useEffect(() => {
    // Check screen width and set showImage state accordingly
    const handleResize = () => {
      setShowImage(window.innerWidth > 350);
    };

    // Initial check
    handleResize();

    // Listen for window resize events
    window.addEventListener('resize', handleResize);

    // Clean up event listener
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, []);

  return (
    <Scaffold style={{ background: '#242DA8' }}>
      <div className="relative flex flex-col h-screen items-center px-4 overflow-hidden">
        <div className="flex flex-col items-center h-auto mt-10">
          <p className="text-white text-2xl text-center font-bold">
            Share this Consent Code to Medical Services
          </p>
          <p className="text-white text-7xl my-6 font-bold">
            {consent ?? '000000'}
          </p>
          <p className="text-white">
            Refresh the code in <span>28</span> second
          </p>

          <p className="underline text-yellow-500 my-6">Get new code</p>

          <BasicButton
            label={'Back'}
            labelStyle={{ color: 'gray' }}
            onPress={() => {
              router.back();
            }}
            classStyle="bg-white rounded-xl py-4 px-12"
          />
        </div>
        {showImage ? (
          <img
            src={Images.consentCode}
            alt=""
            className="absolute -bottom-10"
          />
        ) : null}
      </div>
    </Scaffold>
  );
};

export default ConsentCodePage;
