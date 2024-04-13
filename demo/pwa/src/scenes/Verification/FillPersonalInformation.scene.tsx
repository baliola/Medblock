import { useRouter } from 'next/router';
import React from 'react';

import PrimaryButton from '@/components/Button/PrimaryButton';
import InputText from '@/components/input/InputText';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const FillPersonalInformationPage = () => {
  const router = useRouter();

  return (
    <Scaffold
      topBar={
        <div className="items-center w-screen flex justify-center my-4 bg-white">
          <img src={Images.logoPassport} alt="" className="max-w-[80px]" />
        </div>
      }
      bottomChild={
        <div className="px-8 py-4">
          <PrimaryButton
            title="Submit"
            onSubmit={() => router.push('/verified')}
          />
        </div>
      }
    >
      <div className="w-screen flex flex-col justify-between items-center mb-72 mt-24 px-6">
        <p className="text-gray-800 mt-4 mb-6 font-bold text-sm">
          Verify your ID, Please fill the information below
        </p>

        <InputText
          value=""
          onChange={(e) => {}}
          label="Full Name"
          classStyle="mb-4"
        />
        <InputText
          value=""
          onChange={(e) => {}}
          label="Valid Identity Number"
          classStyle="mb-4"
        />
        <InputText
          value=""
          onChange={(e) => {}}
          label="Address"
          classStyle="mb-4"
        />
        <InputText
          value=""
          onChange={(e) => {}}
          label="Phone Number"
          classStyle="mb-4"
        />
        <InputText
          value=""
          onChange={(e) => {}}
          label="Upload Your ID Card"
          classStyle="mb-4"
          type="file"
        />
        <div className="relative flex items-start">
          <div className="flex h-6 items-center">
            <input
              id="comments"
              aria-describedby="comments-description"
              name="comments"
              type="checkbox"
              className="h-4 w-4 rounded border-gray-300 text-primary-normal focus:ring-primary-ntext-primary-normal"
            />
          </div>
          <div className="ml-3 text-sm leading-6">
            <span id="comments-description" className="text-gray-500">
              <a href="" className="underline">
                I Agree to term of service’s Medblock and all of information
              </a>
            </span>
          </div>
        </div>
      </div>
    </Scaffold>
  );
};

export default FillPersonalInformationPage;
