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
    className={`h-14 outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40 w-full ${
      disable ? 'hover:bg-opacity-80' : ''
    }  `}
    type="submit"
    onClick={onSubmit}
    disabled={disable}
  >
    {title ? (
      <p className="text-center text-white font-bold">{title}</p>
    ) : (
      child
    )}
  </button>
);

export default PrimaryButton;
