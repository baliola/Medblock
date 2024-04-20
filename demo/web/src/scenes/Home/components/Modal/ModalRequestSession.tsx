import Modal from '@/components/Modal/Modal';
import useCountdown from '@/hooks/useCoundown';
import usePatient, { ClaimConsentRequest } from '@/hooks/usePatient';
import { PlusIcon, XCircleIcon, XMarkIcon } from '@heroicons/react/20/solid';
import { AxiosError } from 'axios';
import { ScanBarcode } from 'iconsax-react';
import React, {
  Dispatch,
  SetStateAction,
  useEffect,
  useRef,
  useState,
} from 'react';
import { toast } from 'react-toastify';

interface Modals {
  setShowModal: Dispatch<SetStateAction<boolean>>;
  setShowModalSuccess: Dispatch<SetStateAction<boolean>>;
  toggle: () => void;
}

let currentOTPIndex = 0;
const ModalAddGetPatientSession = ({
  setShowModal,
  setShowModalSuccess,
}: Modals) => {
  const [otp, setOtp] = useState<string[]>(new Array(6).fill(''));
  const [activeOTPIndex, setActiveOTPIndex] = useState<number>(0);
  const [errOtp, setErrOtp] = useState('');
  const { secondLeft, start } = useCountdown();
  const [correctOTP, setCorrectOtp] = useState('');
  const { claimConsent, claimConsentToGetSession } = usePatient();
  const [loading, setLoading] = useState(false);

  const toggleModalPhone = () => {
    setShowModal(false);
  };

  const inputRef = useRef<HTMLInputElement>(null);

  const handleOnChange = ({
    target,
  }: React.ChangeEvent<HTMLInputElement>): void => {
    const { value } = target;
    const newOTP: string[] = [...otp];
    newOTP[currentOTPIndex] = value.substring(value.length - 1);

    if (!value) setActiveOTPIndex(currentOTPIndex - 1);
    else setActiveOTPIndex(currentOTPIndex + 1);

    setOtp(newOTP);
  };

  const handleOnKeyDown = (
    { key }: React.KeyboardEvent<HTMLInputElement>,
    index: number,
  ) => {
    currentOTPIndex = index;
    if (key === 'Backspace') setActiveOTPIndex(currentOTPIndex - 1);
  };

  const handleSendOTP = async () => {
    try {
      setLoading(true);
      const newData: ClaimConsentRequest = {
        code: otp.join(''),
      };
      claimConsentToGetSession(newData).then(() => {
        setShowModal(false);
        setShowModalSuccess(true);
      });
    } catch (error) {
      setLoading(false);
    }
  };

  useEffect(() => {
    inputRef.current?.focus();
  }, [activeOTPIndex]);

  useEffect(() => {
    start(59);
  }, []);

  return (
    <>
      <div className="flex flex-col justify-center items-center rounded-[4px] max-w-[500px] w-full bg-white">
        <div className="flex justify-between w-full p-5 gap-5 border-b">
          <div className="text-slate-600">Request Detail Patient</div>
          <button onClick={toggleModalPhone} className="text-slate-400">
            <XCircleIcon className="w-4 h-4 hover:text-red-500" />
          </button>
        </div>
        <div className="">
          <form action="">
            <div className="px-2 pt-4 text-center text-slate-600 text-sm ">
              You want to access medical record of Mr. Agus Susanto, Please
              input 6 consent code from patient to access the medical record
            </div>
            <div className="p-10 space-y-2">
              <div className="flex justify-center space-x-2">
                {otp.map((_, index) => {
                  return (
                    <div key={index}>
                      <input
                        ref={index === activeOTPIndex ? inputRef : null}
                        type="number"
                        //   className="ps-[16px] md:ps-[20px] p-2 border w-10 md:w-12 rounded-md spin-button-none"
                        className="w-9 h-9 md:w-14 md:h-12 border-b-2 bg-transparent outline-none text-center font-semibold text-xl border-gray-200 focus:border-gray-500 focus:text-gray-700 text-gray-400 transition-all spin-button-none"
                        onChange={handleOnChange}
                        onKeyDown={(e) => handleOnKeyDown(e, index)}
                        value={otp[index]}
                        style={{
                          appearance: 'textfield',
                          WebkitAppearance: 'none',
                        }} // Add this style
                      />
                    </div>
                  );
                })}
              </div>
              {errOtp && (
                <div className="text-sm text-rose-500 text-center">
                  {errOtp}
                </div>
              )}
              <div className="text-center">
                {/* {secondLeft > 0 && (
                  <p>
                    You can resend the code in{' '}
                    <span className="text-[#863DEB]">{secondLeft}</span> seconds
                  </p>
                )} */}
              </div>
            </div>
            <div className="flex justify-center w-full p-6 space-x-3">
              {loading ? (
                <button
                  className="px-4 py-1 text-base font-light rounded-2xl bg-[#42489e] text-white  transition-all ease-in duration-200 "
                  type="button"
                  disabled={true}
                >
                  Loading...
                </button>
              ) : (
                <button
                  className="px-4 py-1 text-base font-light rounded-2xl bg-[#242DA8] text-white  transition-all ease-in duration-200 "
                  type="button"
                  onClick={handleSendOTP}
                >
                  Submit
                </button>
              )}
            </div>
          </form>
        </div>
      </div>
    </>
  );
};

export default ModalAddGetPatientSession;
