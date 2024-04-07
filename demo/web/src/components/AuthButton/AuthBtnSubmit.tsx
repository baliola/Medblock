import { Formik } from 'formik';
import { FC } from 'react';

export interface AuthBtnSubmitProps {
  title: string;
  onSubmit: () => void;
  disable: boolean;
  color: string;
}

const AuthBtnSubmit: FC<AuthBtnSubmitProps> = ({
  title,
  onSubmit,
  disable,
  color,
}) => (
  <button
    className={`${
      title === 'Create' ? 'w-auto px-6' : 'w-[100%]'
    } h-14 outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40 ${
      disable ? 'hover:bg-opacity-80' : ''
    }  `}
    type="submit"
    onClick={onSubmit}
    disabled={disable}
  >
    {disable ? 'Loading...' : title}
  </button>
);

export default AuthBtnSubmit;
