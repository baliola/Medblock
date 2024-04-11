import RegistrationsForm from '@/components/Form/RegistrationsForm';
import { googleIcon, eidLogo } from '@/lib/assets';
import { NextPageWithLayout } from '@/types';
import { Link } from 'iconsax-react';
import React from 'react';
const RegistrationsScene: NextPageWithLayout = () => {
  return (
    <div className="flex w-full">
      <div className="flex flex-col w-1/2 items-center relative">
        <img
          src={'/assets/hero.svg'}
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
          src={'/assets/hospital.svg'}
          alt=""
          className="absolute max-w-[300px] right-24 -top-4 w-full"
        />
        <img
          src={'/assets/healthMinister.svg'}
          alt=""
          className="absolute max-w-[300px] bottom-32 right-4 w-full"
        />
        <img
          src={'/assets/umbrella.svg'}
          alt=""
          className="absolute max-w-[300px] bottom-4 right-auto w-full"
        />
        <div className="flex flex-col w-full relative items-center justify-center h-full">
          <div className="flex relative flex-col mx-auto justify-center shadow-lg border-white border-[2px]  rounded-2xl backdrop-blur-md bg-white/65  items-center px-6 py-[38px] max-w-[461px] w-full">
            <RegistrationsForm
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

export default RegistrationsScene;
RegistrationsScene.disableLayout = true;
