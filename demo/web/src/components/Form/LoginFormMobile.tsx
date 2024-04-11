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
import useMobile from '@/hooks/useMobile';

// import AuthBtnSubmit from '../Button/AuthButton/AuthBtnSubmit';
// import loginValidationSchema from '@/lib/faker/validation/auth/LoginValidation';
// import { Assets } from '@/constant/generated/assets';
// import useAuth from '@/hooks/useAuth';

export interface LoginFormMobileProps {
  titleEmail: string;
  titlePassword: string;
  placeholderEmail: string;
  placeholderPassword: string;
  required: boolean;
}

const LoginFormMobile: FC<LoginFormMobileProps> = ({
  titleEmail,
  titlePassword,
  placeholderEmail,
  placeholderPassword,
  required = false,
}) => {
  // const {
  //   formData,
  //   showError,
  //   showPassword,
  //   setFormData,
  //   setShowError,
  //   setShowPassword,
  //   handleLogin,
  // } = useAuth();
  const [formData, setFormData] = useState({
    email: '',
    password: '',
  });
  const [showError, setShowError] = useState(false);

  const { handleLogin } = useMobile();

  useEffect(() => {
    localStorage.removeItem('user');
  }, []);

  return (
    <Formik
      initialValues={formData}
      // validationSchema={''}
      onSubmit={handleLogin}
    >
      {({ errors, handleChange, handleSubmit, isSubmitting, values }) => (
        <>
          <div className="flex flex-col gap-2 justify-center items-center text-center mb-[30px]">
            {/* <Image src={Assets.logo} width={120} height={120} alt={''} /> */}
            <p className="text-center font-semibold text-6xl">
              Med<span className="text-[#F04438]">block</span>
            </p>
            <p className='"text-center text-2xl font-medium '>
              Secure Health, Seamless Care{' '}
            </p>
          </div>

          <div className="flex flex-col w-full px-[16px] gap-4">
            <div className="flex justify-end w-[100%]"></div>
            <AuthBtnSubmit
              title="Sign in"
              onSubmit={handleSubmit}
              disable={false}
              color="#242DA8"
            />
            <div className="flex w-full justify-center gap-2">
              <img
                src={'/assets/line.svg'}
                alt=""
                className="w-full max-w-[104px]"
              />
              <p className="text-center font-medium text-base relative text-[#5D5D5D]">
                OR CONTINUE WITH
              </p>
              <img
                src={'/assets/line.svg'}
                alt=""
                className="w-full max-w-[104px]"
              />
            </div>
            <div className="flex gap-4">
              <button className="h-14 border-[2px] w-full p-4 items-center flex justify-center rounded-2xl border-[#242DA8]">
                <img src={googleIcon} alt="" />
              </button>
              <button className="h-14 border-[2px] w-full p-4 items-center flex justify-center rounded-2xl border-[#242DA8]">
                <img src={eidLogo} alt="" />
              </button>
            </div>
            <div className="flex flex-col gap-4 justify-center">
              <button className="h-14 gap-4 border-[2px] w-full p-4 items-center flex justify-center rounded-2xl border-[#242DA8]">
                <img src={'/assets/passkeyIcon.svg'} alt="" />
                <p className="text-base font-medium">Continue With Passkey</p>
              </button>
              <Link href={''} className="text-[#06B8EE] underline text-center">
                Other Sign Option
              </Link>
            </div>
          </div>
        </>
      )}
    </Formik>
  );
};

export default LoginFormMobile;
