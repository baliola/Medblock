import { useRouter } from 'next/router';
import React from 'react';

import PrimaryButton from '@/components/Button/PrimaryButton';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const VerifiedPage = () => {
  const router = useRouter();

  return (
    <Scaffold>
      <div className="h-screen w-screen flex flex-col justify-center items-center p-6">
        <div className="flex flex-col justify-center items-center my-10">
          <img src={Images.verified} alt="" className="max-w-[200px] my-4" />
          <p className="text-xl font-bold">Your account is verified!</p>
          <p className="mb-4 text-center text-sm">
            Now you can share and control your medical record to hospital.
          </p>
        </div>
        <PrimaryButton
          onSubmit={() => {
            router.push('/home');
          }}
          title="Continue"
        />
      </div>
    </Scaffold>
  );
};

export default VerifiedPage;
