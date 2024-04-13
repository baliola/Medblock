import {
  CheckBadgeIcon,
  CheckCircleIcon,
  CheckIcon,
} from '@heroicons/react/20/solid';
import XCircleIcon from '@heroicons/react/20/solid/XCircleIcon';
import React from 'react';

type ModalSuccessType = {
  toggle: () => void;
};
export default function ModalSuccess(props: ModalSuccessType) {
  return (
    <div className="flex flex-col justify-center items-center rounded-[4px] max-w-[500px] w-full bg-white">
      <div className="flex flex-col justify-center items-center px-5">
        <form action="">
          <div className="flex-col flex justify-center items-center px-2 pt-4 text-center text-slate-600 text-sm ">
            <CheckCircleIcon className="text-[#242DA8] w-32" />
            <p>Congratulation! Now you have access to Mr. Agus Susanto EMR.</p>
          </div>

          <div className="flex justify-center w-full p-6 space-x-3">
            <button
              className="px-4 py-1 text-base font-light rounded-2xl bg-[#242DA8] text-white  transition-all ease-in duration-200 "
              type="button"
              onClick={props.toggle}
            >
              Close
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
