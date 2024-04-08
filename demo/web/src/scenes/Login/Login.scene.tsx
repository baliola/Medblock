import {
  Card,
  Grid,
  Tab,
  TabGroup,
  TabList,
  TabPanel,
  TabPanels,
  Text,
  Title,
} from '@tremor/react';
import { useState } from 'react';

import { NextPageWithLayout } from '@/types';
import LoginForm from '@/components/Form/LoginForm';
import Image from 'next/image';
import {
  healthMinisterImage,
  heroImage,
  hospitalImage,
  umbrellaImage,
} from '@/lib/assets';

const LoginPage: NextPageWithLayout = () => {
  const [passwordVisible, setPasswordVisible] = useState(false);
  const [password, setPassword] = useState('');

  const togglePasswordVisibility = () => {
    setPasswordVisible(!passwordVisible);
  };

  return (
    <div className="flex w-full">
      <div className="flex flex-col w-1/2 items-center relative">
        <img
          src={heroImage}
          alt=""
          className="relative left-12  max-w-[740px] w-full"
        />
        <p className="text-5xl max-w-[378px] text-center font-bold">
          Your <span className="text-[#D30837]">Medical</span> Record On Your
          Own <span className="text-blue-600">Block</span>
        </p>
      </div>
      <div className="flex w-1/2 ">
        <img
          src={hospitalImage}
          alt=""
          className="absolute max-w-[300px] right-24 -top-4 w-full"
        />
        <img
          src={healthMinisterImage}
          alt=""
          className="absolute max-w-[300px] bottom-32 right-4 w-full"
        />
        <img
          src={umbrellaImage}
          alt=""
          className="absolute max-w-[300px] bottom-4 right-auto w-full"
        />
        <div className="flex flex-col w-full relative items-center justify-center h-full">
          <div className="flex relative flex-col mx-auto justify-center shadow-lg border-white border-[2px]  rounded-2xl backdrop-blur-md bg-white/65  items-center px-6 py-[38px] max-w-[461px] w-full">
            <LoginForm
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

export default LoginPage;
LoginPage.disableLayout = true;
