import { useRouter } from 'next/router';
import React from 'react';

import PrimaryButton from '@/components/Button/PrimaryButton';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const UnverifiedPage = () => {
  const router = useRouter();

  return (
    <Scaffold>
      <div className="h-screen w-screen flex flex-col justify-center items-center p-6">
        <img src={Images.logoPassport} alt="" className="max-w-[100px]" />
        <div className="flex flex-col justify-center items-center my-10">
          <p className="text-bold text-xl font-bold">Welcome</p>
          <img
            src={Images.dummyProfile}
            alt=""
            className="max-w-[200px] my-4"
          />
          <p className="text-bold text-xl mb-4 font-bold">I Putu Agus</p>
        </div>
        <PrimaryButton
          onSubmit={() => {
            router.push('/verification');
          }}
          title="Verify Your ID"
        />
      </div>
    </Scaffold>
  );
};

export default UnverifiedPage;
