import { useRouter } from 'next/router';
import React from 'react';

import PrimaryButton from '@/components/Button/PrimaryButton';
import { useAuth } from '@/config/agent';
import Images from '@/constants/images';
import useAuthentication from '@/hooks/useAuth';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';
import { useCentralStore } from '@/Store';

const LoginPage = () => {
  const router = useRouter();
  const { setClient, setUserPrincipal } = useCentralStore();
  const { handleAuthenticate, handleLogin, handleLoginII } =
    useAuthentication();
  const { identity } = useAuth();

  return (
    <Scaffold>
      <div className="h-screen w-screen flex flex-col justify-center items-center p-6 gap-4">
        <img src={Images.logoPassport} alt="" className="max-w-[200px] mb-10" />
        <PrimaryButton
          onSubmit={() => {
            handleLogin();
          }}
          title="Login"
        />

        {/* <PrimaryButton
          onSubmit={() => {
            handleLoginII();
          }}
          title="Login with Internet Identity"
        /> */}
      </div>
    </Scaffold>
  );
};

export default LoginPage;
