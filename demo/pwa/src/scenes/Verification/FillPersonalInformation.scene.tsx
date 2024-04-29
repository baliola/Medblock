import { useRouter } from 'next/router';
import React, { useState } from 'react';
import { toast } from 'react-toastify';

import PrimaryButton from '@/components/Button/PrimaryButton';
import InputText from '@/components/input/InputText';
import Images from '@/constants/images';
import usePatient from '@/hooks/usePatient';
import { createCanisterError } from '@/interface/CanisterError';
import { ErrorMessages } from '@/interface/constant';
import RegisterRequest from '@/interface/register_request';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';
import InputDropdown from '@/components/input/InputDropdown';
import { Formik } from 'formik';
import registrationValidation from '@/lib/validations/registerValidations';

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

  const handleSubmit = async (values: RegisterRequest) => {
    console.log('valuess', values);
    console.log('valuess formdata', formData);
    const data: RegisterRequest = {
      nik: values.nik,
      name: values.name,
      martial_status: values.martial_status,
      place_of_birth: values.place_of_birth,
      address: values.address,
      gender: values.gender,
      date_of_birth: values.date_of_birth,
    };

    setLoading(true);
    try {
      await registerPatient(data);
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
    <Formik
      initialValues={formData}
      validationSchema={registrationValidation}
      validateOnBlur={false}
      onSubmit={handleSubmit}
    >
      {({
        errors,
        handleChange,
        handleSubmit,
        isSubmitting,
        values,
        setFieldValue,
      }) => (
        <Scaffold
          loading={loading}
          topBar={
            <div className="items-center w-screen flex justify-center my-4 bg-white">
              <img src={Images.logoPassport} alt="" className="max-w-[80px]" />
            </div>
          }
          bottomChild={
            <div className="px-8 py-4">
              <PrimaryButton title="Submit" onSubmit={handleSubmit} />
            </div>
          }
        >
          <div className="w-screen flex flex-col justify-between items-center mb-72 mt-24 px-6 pb-48">
            <p className="text-gray-800 mt-4 mb-6 font-bold text-sm">
              Verify your ID, Please fill the information below
            </p>
            <InputText
              value={formData.name}
              name={'name'}
              id="name"
              type="text"
              onChange={handleChange}
              label="Full Name"
              classStyle="mb-2"
            />
            {errors?.name && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.name}
              </p>
            )}{' '}
            <InputText
              value={formData.address}
              name={'address'}
              id="address"
              type="text"
              onChange={handleChange}
              label="Address"
              classStyle="mb-2"
            />
            {errors?.address && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.address}
              </p>
            )}{' '}
            <InputText
              value={formData.nik}
              name={'nik'}
              id="nik"
              type="text"
              onChange={handleChange}
              label="Valid Identity Number"
              classStyle={`mb-2 `}
            />
            {errors?.nik && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.nik}
              </p>
            )}{' '}
            <InputDropdown
              value={formData.gender}
              name={'gender'}
              id="gender"
              onChange={handleChange}
              placeholder="Please select your gender"
              label="Gender"
              options={['Male', 'Female', 'Others']}
              classStyle="mb-2"
            />
            {errors?.gender && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.gender}
              </p>
            )}{' '}
            <InputText
              value={formData.date_of_birth}
              name={'date_of_birth'}
              id="date_of_birth"
              onChange={handleChange}
              label="Birth Date"
              classStyle="mb-2"
              type="date"
            />
            {errors?.date_of_birth && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.date_of_birth}
              </p>
            )}{' '}
            <InputText
              value={formData.place_of_birth}
              name={'place_of_birth'}
              id="place_of_birth"
              onChange={handleChange}
              label="Place of Date"
              classStyle="mb-2"
            />
            {errors?.place_of_birth && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.place_of_birth}
              </p>
            )}{' '}
            <InputDropdown
              value={formData.martial_status}
              name={'martial_status'}
              id="martial_status"
              onChange={handleChange}
              label="Marital Status"
              placeholder="Please select your marital status"
              options={['Single', 'Married', 'Divorced', 'Widowed']}
              classStyle="mb-2"
            />
            {errors?.martial_status && (
              <p className="w-full mb-2 text-start text-[#F04438]">
                {errors?.martial_status}
              </p>
            )}{' '}
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
      )}
    </Formik>
  );
};

export default FillPersonalInformationPage;
