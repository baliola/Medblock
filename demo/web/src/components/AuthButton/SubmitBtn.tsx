import { Formik } from 'formik';
import { FC } from 'react';

export interface BtnSubmitProps {
  title: string;
  onSubmit: () => void;
  disable: boolean;
  color: string;
}

const BtnSubmit: FC<BtnSubmitProps> = ({ title, onSubmit, disable, color }) => (
  <button
    className={`${
      title === 'Create' ? 'w-auto px-6' : ''
    } h-14 outline-hover justify-center border-[2px] outline-hover align-middle px-2 bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40 ${
      disable ? 'hover:bg-opacity-80' : ''
    }  `}
    type="submit"
    onClick={onSubmit}
    disabled={disable}
  >
    {disable ? 'Loading...' : title}
  </button>
);

export default BtnSubmit;
