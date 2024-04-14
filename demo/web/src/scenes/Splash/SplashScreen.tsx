import React from 'react';

export default function SplashScreen() {
  return (
    <div className="flex justify-center  bg-no-repeat w-full bg-cover items-center min-h-screen p-8 my-auto bg-bg-primary text-text-primary ">
      <div className="flex flex-col items-center h-full gap-10">
        <img src="/assets/logo.svg" alt="" />
      </div>
    </div>
  );
}
