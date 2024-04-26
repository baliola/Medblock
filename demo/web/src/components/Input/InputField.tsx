import React, { useState } from 'react';
import { Field, FormikErrors } from 'formik';

type InputFieldProps = {
  fieldName: string;
  label?: string;
  handleChange?: (field: string) => void;
  errors: FormikErrors<any>;
  isPassword: boolean;
  isNumber?: boolean;
  isDisable?: boolean;
  values?: any;
};

const InputField: React.FC<InputFieldProps> = ({
  fieldName,
  handleChange,
  errors,
  isPassword,
  isNumber,
  label,
  isDisable,
  values,
}) => {
  const [showPassword, setShowPassword] = useState(false);

  return (
    <div className="w-full ">
      <p className="capitalize text-gray-500">{label ? label : fieldName}</p>
      {isPassword ? (
        <div
          className={`flex flex-flex gap-0 border rounded-tr-[3px] rounded-br-[3px] rounded-tl-[3px] rounded-bl-[3px]`}
        >
          {/* <div className="flex px-2 w-full"> */}
          <Field
            id="password"
            name="password"
            type={showPassword ? 'text' : 'password'}
            className="w-full h-[34px] pl-4 rounded-tl-[3px] border-r-0 rounded-bl-[3px] text-[14px]"
            onChange={handleChange}
          />
          {/* </div> */}
          <button
            className="bg-[#E6C7FF] w-[34px] h-[34px] flex justify-center items-center border-l border-[#B4BAC6] rounded-tr-[3px] rounded-br-[3px]"
            onClick={() => setShowPassword(!showPassword)}
          >
            <img width={18} height={18} src="/asset/images/eye.svg" alt="" />
          </button>
        </div>
      ) : (
        <>
          {values ? (
            <Field
              name={fieldName}
              onChange={handleChange}
              type={isNumber ? 'number' : fieldName}
              placholder={label}
              disabled={isDisable ? isDisable : false}
              value={values}
              className={`capitalize mt-[6px] flex flex-row min-w-full h-[34px] p-6 text-[14px] border rounded-md px-4 border-[#B4BAC6] focus:outline-none focus:border-primary focus:border-b-[1px] ${
                isDisable ? 'text-gray-400' : ''
              }`}
            />
          ) : (
            <Field
              name={fieldName}
              onChange={handleChange}
              type={isNumber ? 'number' : fieldName}
              placholder={label}
              disabled={isDisable ? isDisable : false}
              // value={values}
              className={`mt-[6px] flex flex-row min-w-full h-[34px] p-6 text-[14px] border rounded-md px-4 border-[#B4BAC6] focus:outline-none focus:border-primary focus:border-b-[1px] ${
                isDisable ? 'text-gray-400' : ''
              }`}
            />
          )}
        </>
      )}

      {/* {errors} */}
    </div>
  );
};

export default InputField;
