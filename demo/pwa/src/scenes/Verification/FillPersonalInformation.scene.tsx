import { useRouter } from 'next/router';
import React, { useState } from 'react';
import { toast } from 'react-toastify';

import PrimaryButton from '@/components/Button/PrimaryButton';
import InputText from '@/components/input/InputText';
import Images from '@/constants/images';
import usePatient from '@/hooks/usePatient';
import { createCanisterError } from '@/interface/CanisterError';
import { ErrorMessages } from '@/interface/constant';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const FillPersonalInformationPage = () => {
  const router = useRouter();
  const [loading, setLoading] = useState<boolean>(false);
  const { registerPatient } = usePatient();
  const [formData, setFormData] = useState<RegisterRequest>({
    name: '',
    address: '',
    date_of_birth: '',
    gender: '',
    martial_status: '',
    nik: '',
    place_of_birth: '',
  });

  const handleSubmit = async () => {
    setLoading(true);
    try {
      await registerPatient(formData);
      setLoading(false);
    } catch (error) {
      setLoading(false);
      const canisterError = createCanisterError(error);
      if (canisterError?.message.includes(ErrorMessages.ProviderDoesNotExist)) {
        toast.error(canisterError.message);
      } else {
        console.log('-----------------');
        console.log('ERROR::::', error);
      }
      console.log('====================================');
      console.log('ERROR --> ', error);
      console.log('====================================');
    }
  };

  return (
    <Scaffold
      loading={loading}
      topBar={
        <div className="items-center w-screen flex justify-center my-4 bg-white">
          <img src={Images.logoPassport} alt="" className="max-w-[80px]" />
        </div>
      }
      bottomChild={
        <div className="px-8 py-4">
          <PrimaryButton
            title="Submit"
            onSubmit={() => {
              handleSubmit();
            }}
          />
        </div>
      }
    >
      <div className="w-screen flex flex-col justify-between items-center mb-72 mt-24 px-6 pb-48">
        <p className="text-gray-800 mt-4 mb-6 font-bold text-sm">
          Verify your ID, Please fill the information below
        </p>

        <InputText
          value={formData.name}
          onChange={(e) => {
            setFormData({ ...formData, name: e.target.value });
          }}
          label="Full Name"
          classStyle="mb-4"
        />
        <InputText
          value={formData.nik}
          onChange={(e) => {
            setFormData({ ...formData, nik: e.target.value });
          }}
          label="Valid Identity Number"
          classStyle={`mb-4 ${
            formData.nik.length !== 16 ? 'border-red-500' : ''
          }`}
          isError={formData.nik.length !== 16}
          error="Valid Identity Number must be exactly 16 characters long."
        />
        {/* <div> */}

        {/* {formData.nik.length !== 16 && (
          <p className="text-red-500">
            Valid Identity Number must be exactly 16 characters long.
          </p>
        )} */}
        {/* </div> */}
        <InputText
          value={formData.gender}
          onChange={(e) => {
            setFormData({ ...formData, gender: e.target.value });
          }}
          label="Gender"
          classStyle="mb-4"
        />
        <InputText
          value={formData.date_of_birth}
          onChange={(e) => {
            setFormData({ ...formData, date_of_birth: e.target.value });
          }}
          label="Birth Date"
          classStyle="mb-4"
          type="date"
        />
        <InputText
          value={formData.place_of_birth}
          onChange={(e) => {
            setFormData({ ...formData, place_of_birth: e.target.value });
          }}
          label="Place Date"
          classStyle="mb-4"
        />
        <InputText
          value={formData.martial_status}
          onChange={(e) => {
            setFormData({ ...formData, martial_status: e.target.value });
          }}
          label="Martial Status"
          classStyle="mb-4"
          isError={formData.martial_status.length > 10}
          error="Martioal must be less than 10 characters long."
        />
        {/* <InputText
          value=""
          onChange={(e) => {}}
          label="Upload Your ID Card"
          classStyle="mb-4"
          type="file"
        /> */}
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
