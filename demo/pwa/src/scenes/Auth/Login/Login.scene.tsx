import { useRouter } from 'next/router';
import React from 'react';

import PrimaryButton from '@/components/Button/PrimaryButton';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const LoginPage = () => {
  const router = useRouter();

  return (
    <Scaffold>
      <div className="h-screen w-screen flex flex-col justify-center items-center p-6">
        <img src={Images.logoPassport} alt="" className="max-w-[200px] mb-10" />
        <PrimaryButton
          onSubmit={() => {
            router.push('/unverified');
          }}
          title="Login"
        />
      </div>
    </Scaffold>
  );
};

export default LoginPage;
