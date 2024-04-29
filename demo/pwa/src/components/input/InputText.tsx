import React from 'react';

interface InputTextProps {
  id?: string;
  label?: string;
  placeholder?: string;
  classStyle?: string;
  isSecure?: boolean;
  value: string;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  prefix?: React.ReactElement<any, any>;
  suffix?: React.ReactElement<any, any>;
  error?: string;
  enable?: boolean;
  isError?: boolean;
  autoFocus?: boolean;
  outlined?: boolean;
  numberOfLine?: number;
  name?: string;
  type?: React.HTMLInputTypeAttribute;
}

const InputText: React.FC<InputTextProps> = ({
  id,
  label,
  name,
  placeholder,
  classStyle,
  isSecure = false,
  value,
  prefix,
  suffix,
  error,
  enable = true,
  autoFocus = false,
  onChange,
  outlined = false,
  numberOfLine,
  isError,
  type,
}) => {
  return (
    <div className="w-full">
      <label
        htmlFor="name"
        className="block text-sm font-medium leading-6 text-gray-900"
      >
        {label}
      </label>
      <div className="mt-2">
        <input
          type={type}
          // value={value}
          name={name}
          onChange={onChange}
          id={id}
          className={
            'block w-full rounded-xl border-0 px-4 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-[#242DA8] sm:text-sm sm:leading-6 ' +
            classStyle
          }
          placeholder={placeholder}
        />
      </div>
    </div>
  );
};

export default InputText;
