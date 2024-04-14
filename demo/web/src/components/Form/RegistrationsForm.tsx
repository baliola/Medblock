import { AxiosError } from 'axios';
import Image from 'next/image';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { FC, useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { ErrorMessage, Field, Formik, useFormik } from 'formik';
import AuthBtnSubmit from '../AuthButton/AuthBtnSubmit';
import { Identity } from '@dfinity/agent';
import { NFID } from '@nfid/embed';
import { NFIDConfig } from '@nfid/embed/src/lib/types';
import { eidLogo, googleIcon, line, passkeyIcon } from '@/lib/assets';
import { AuthClient } from '@dfinity/auth-client';
import { EyeDropperIcon, EyeIcon } from '@heroicons/react/20/solid';
import { Eye, EyeSlash } from 'iconsax-react';
import { useCentralStore } from '@/Store';
import { targetCanister } from '@/lib/canister/target.canister';
import { emrCanisterId } from '@/lib/canister/emr.canister';
import { patientCanisterId } from '@/lib/canister/patient.canister';
import { providerCanisterId } from '@/lib/canister/provider.canister';
import useAuth from '@/hooks/useAuth';
import RegisterProviderValidations from '@/lib/validation/Provider.validation';
import useProvider from '@/hooks/useProvider';
import { RegisterProviderRequest } from '@/interface/Provider.interface';
import useRegister from '@/hooks/useRegister';

// import AuthBtnSubmit from '../Button/AuthButton/AuthBtnSubmit';
// import loginValidationSchema from '@/lib/faker/validation/auth/LoginValidation';
// import { Assets } from '@/constant/generated/assets';
// import useAuth from '@/hooks/useAuth';

export interface RegistrationsFormProps {
  titleEmail: string;
  titlePassword: string;
  placeholderEmail: string;
  placeholderPassword: string;
  required: boolean;
}

const RegistrationsForm: FC<RegistrationsFormProps> = ({
  titleEmail,
  titlePassword,
  placeholderEmail,
  placeholderPassword,
  required = false,
}) => {
  const [formData, setFormData] = useState({
    displayName: '',
    address: '',
  });
  const { registerProvider } = useRegister();
  //
  const handleRegister = (values: RegisterProviderRequest) => {
    console.log('values', values);
    const data: RegisterProviderRequest = {
      displayName: values.displayName,
      address: values.address,
    };
    registerProvider(data);
  };

  useEffect(() => {
    localStorage.removeItem('user');
  }, []);

  return (
    <Formik
      initialValues={formData}
      validationSchema={RegisterProviderValidations}
      onSubmit={handleRegister}
    >
      {({ errors, handleChange, handleSubmit, isSubmitting, values }) => (
        <>
          <div className="flex flex-col gap-2 justify-center items-center mb-[30px]">
            {/* <Image src={Assets.logo} width={120} height={120} alt={''} /> */}
            <p className="text-center font-semibold text-6xl">
              Med<span className="text-[#F04438]">block</span>
            </p>
            <p className='"text-center text-2xl font-medium '>
              Register your hospital
            </p>
          </div>

          <div className="flex flex-col w-full px-[16px] gap-4">
            <div className={`flex flex-col gap-2`}>
              <div className="grid grid-cols-2">
                <label htmlFor="">
                  {' '}
                  <span className="block text-sm font-normal text-slate-600">
                    Hospital Name
                  </span>
                </label>
              </div>
              <input
                id="displayName"
                name="displayName"
                type={'text'}
                placeholder={'Enter your hospital name...'}
                onChange={handleChange('displayName')}
                className={
                  'rounded-2xl bg-[#E7E7E7]  mt-[6px] flex flex-row min-w-full h-[56px] rounde text-[14px] border px-4  border-[#B4BAC6] focus:outline-none focus:border-[#397BFF] focus:border-b-[1px]'
                }
              />
              {errors?.displayName && (
                <p className="text-[#F04438]">{errors?.displayName}</p>
              )}{' '}
            </div>{' '}
            <div className={`flex flex-col gap-2`}>
              <div className="grid grid-cols-2">
                <label htmlFor="">
                  {' '}
                  <span className="block text-sm font-normal text-slate-600">
                    Hospital Address
                  </span>
                </label>
              </div>
              <input
                id="address"
                name="address"
                type={'text'}
                placeholder={'Enter your hospital address...'}
                onChange={handleChange('address')}
                className={
                  'rounded-2xl bg-[#E7E7E7]  mt-[6px] flex flex-row min-w-full h-[56px] rounde text-[14px] border px-4  border-[#B4BAC6] focus:outline-none focus:border-[#397BFF] focus:border-b-[1px]'
                }
              />
              {errors?.address && (
                <p className="text-[#F04438]">{errors?.address}</p>
              )}{' '}
            </div>{' '}
            <div className="flex justify-end w-[100%]"></div>
            <AuthBtnSubmit
              title="Register"
              onSubmit={handleSubmit}
              disable={false}
              color="#242DA8"
            />
            <div className="flex flex-col gap-4 justify-center">
              <Link
                href={'/auth/login'}
                className="text-[#06B8EE] underline text-center"
              >
                Back to Login
              </Link>
            </div>
          </div>
        </>
      )}
    </Formik>
  );
};

export default RegistrationsForm;
