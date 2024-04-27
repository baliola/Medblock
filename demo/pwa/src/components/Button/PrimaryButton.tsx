import { FC } from 'react';

export interface PrimaryButtonProps {
  title?: string;
  onSubmit: () => void;
  disable?: boolean;
  child?: React.ReactElement<any, any>;
  classStyle?: string;
}

const PrimaryButton: FC<PrimaryButtonProps> = ({
  title,
  onSubmit,
  disable,
  child,
  classStyle,
}) => (
  <button
    className={`h-14 outline-hover justify-center align-middle     rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40 w-full ${
      disable
        ? 'bg-opacity-50 bg-gray-500 text-white'
        : 'hover:bg-opacity-80 transition-all ease-in duration-200 bg-[#242DA8] text-white'
    }  `}
    type="submit"
    onClick={onSubmit}
    disabled={disable}
  >
    {title ? (
      <p className="text-center text-white font-bold">
        {title}
        {disable}
      </p>
    ) : (
      child
    )}
  </button>
);

export default PrimaryButton;
