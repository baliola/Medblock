import { useState } from 'react';

import { NextPageWithLayout } from '@/types';
import LoginForm from '@/components/Form/LoginForm';
import LoginFormMobile from '@/components/Form/LoginFormMobile';

const LoginPageMobile: NextPageWithLayout = () => {
  const [passwordVisible, setPasswordVisible] = useState(false);
  const [password, setPassword] = useState('');

  const togglePasswordVisibility = () => {
    setPasswordVisible(!passwordVisible);
  };

  return (
    <div className="flex w-full">
      <div className="flex w-full ">
        <div className="flex flex-col w-full relative items-center justify-center h-full">
          <div className="flex relative flex-col mx-auto justify-center shadow-lg border-white border-[2px]  rounded-2xl backdrop-blur-md bg-white/65  items-center px-6 py-[38px] max-w-[461px] w-full">
            <LoginFormMobile
              titleEmail={'Username'}
              titlePassword={'Password'}
              placeholderEmail={'Please input your username'}
              placeholderPassword={'********'}
              required={false}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default LoginPageMobile;
LoginPageMobile.disableLayout = true;
