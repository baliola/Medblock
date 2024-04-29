import { useCentralStore } from '@/Store';
import {
  CheckBadgeIcon,
  CheckCircleIcon,
  CheckIcon,
} from '@heroicons/react/20/solid';
import XCircleIcon from '@heroicons/react/20/solid/XCircleIcon';
import { useRouter } from 'next/router';
import React from 'react';

type ModalSuccessType = {
  toggle: () => void;
  sessionId?: string | null;
};
export default function ModalSuccess(props: ModalSuccessType) {
  const { patientName, isErrorConsent } = useCentralStore();
  const router = useRouter();
  const goToDetail = () => {
    router.push(`/emr/${props.sessionId}`);
  };

  console.log('session id from modal', props.sessionId);

  console.log(props.sessionId);
  return (
    <div
      className={`flex flex-col justify-center items-center rounded-[4px] max-w-[500px] ${
        isErrorConsent ? 'w-[300px]' : 'w-full'
      }  bg-white`}
    >
      <div className="flex flex-col justify-center items-center px-5">
        <form action="">
          <div className="flex-col flex justify-center items-center px-2 pt-4 text-center text-slate-600 text-sm ">
            {isErrorConsent ? (
              <XCircleIcon className="text-red-300 w-32" />
            ) : (
              <CheckCircleIcon className="text-[#242DA8] w-32" />
            )}
            {isErrorConsent ? (
              <p className="w-full">Patient data not found </p>
            ) : (
              <p>
                Congratulation! Now you have access to {patientName ?? ''} EMR.
              </p>
            )}
          </div>

          <div className="flex justify-center w-full p-6 space-x-3">
            {props.sessionId ? (
              <button
                className="px-4 py-1 text-base font-light rounded-2xl bg-[#242DA8] text-white  transition-all ease-in duration-200 "
                type="button"
                onClick={goToDetail}
              >
                Go to Detail
              </button>
            ) : (
              <button
                className="px-4 py-1 text-base font-light rounded-2xl bg-[#242DA8] text-white  transition-all ease-in duration-200 "
                type="button"
                onClick={props.toggle}
              >
                Close
              </button>
            )}
          </div>
        </form>
      </div>
    </div>
  );
}
