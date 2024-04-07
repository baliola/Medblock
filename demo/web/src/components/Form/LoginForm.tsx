import { AxiosError } from 'axios';
import Image from 'next/image';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { FC, useEffect, useState } from 'react';
import { toast } from 'react-toastify';
import { ErrorMessage, Field, Formik, useFormik } from 'formik';
import AuthBtnSubmit from '../AuthButton/AuthBtnSubmit';
import { Identity } from '@dfinity/agent';
import { testNFID } from '@/interface/nfid.interface';
import { NFID } from '@nfid/embed';
import { NFIDConfig } from '@nfid/embed/src/lib/types';
import { eidLogo, googleIcon, line, passkeyIcon } from '@/lib/assets';

// import AuthBtnSubmit from '../Button/AuthButton/AuthBtnSubmit';
// import loginValidationSchema from '@/lib/faker/validation/auth/LoginValidation';
// import { Assets } from '@/constant/generated/assets';
// import useAuth from '@/hooks/useAuth';

export interface LoginFormProps {
  titleEmail: string;
  titlePassword: string;
  placeholderEmail: string;
  placeholderPassword: string;
  required: boolean;
}

const LoginForm: FC<LoginFormProps> = ({
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
  const [showPassword, setShowPassword] = useState(false);

  const targetCanisterIds = 'tvrxx-2iaaa-aaaak-akn4a-cai';

  const handleLogin = async () => {
    console.log('running submit login');
    const targets = targetCanisterIds.length ? [targetCanisterIds] : undefined;

    try {
      const nfid = await NFID.init({
        application: {
          name: 'My Sweet App',
          logo: 'https://dev.nfid.one/static/media/id.300eb72f3335b50f5653a7d6ad5467b3.svg',
        },
      } as NFIDConfig);
      // const identity: Identity = nfid.getIdentity();
      const identity = await nfid.getDelegation({
        targets: targets,
        // You can add other optional properties here if needed
      });
      console.log('response', identity);
      console.log('response nfid', nfid);
    } catch (error) {
      console.log('error', error);
    }
  };

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
          <div className="flex flex-col gap-2 justify-center items-center mb-[30px]">
            {/* <Image src={Assets.logo} width={120} height={120} alt={''} /> */}
            <p className="text-center font-semibold text-6xl">
              Med<span className="text-[#F04438]">block</span>
            </p>
            <p className='"text-center text-2xl font-medium '>
              Secure Health, Seamless Care{' '}
            </p>
          </div>
          {showError && (
            <div className="flex flex-row h-10 w-full items-center bg-[#FEECEB] rounded-xl gap-4 p-5">
              <img src="/assets/Icons/alertError.svg" alt="" />
              <p className="text-[#F04438]">Email atau Password salah!</p>
            </div>
          )}
          <div className="flex flex-col w-full px-[16px] gap-4">
            {/* EMAIL FORM INPUT */}
            <label htmlFor="email" className="flex flex-col w-[100%]">
              <span className="block text-[14px] font-normal text-[#454768]">
                {titleEmail}
              </span>
              <input
                id="email"
                name="email"
                type="email"
                placeholder={placeholderEmail}
                value={values.email}
                onChange={handleChange('email')}
                autoComplete="off"
                className={
                  'rounded-2xl bg-[#E7E7E7]  mt-[6px] flex flex-row min-w-full h-[56px] rounde text-[14px] border px-4  border-[#B4BAC6] focus:outline-none focus:border-[#397BFF] focus:border-b-[1px]'
                }
              />
              {errors?.email && (
                <p className="text-[#F04438] mt-0.5">{errors.email}</p>
              )}

              {/* <ErrorMessage name="email" component="div" /> */}
            </label>
            <div className="flex justify-end w-[100%]"></div>
            <AuthBtnSubmit
              title="Continue With Email"
              onSubmit={handleSubmit}
              disable={false}
              color="#242DA8"
            />
            <div className="flex w-full justify-center gap-2">
              <img src={line} alt="" className="w-full max-w-[104px]" />
              <p className="text-center font-medium text-base relative text-[#5D5D5D]">
                OR CONTINUE WITH
              </p>
              <img src={line} alt="" className="w-full max-w-[104px]" />
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
                <img src={passkeyIcon} alt="" />
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

export default LoginForm;
