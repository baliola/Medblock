import React, { useState } from 'react';
import { Field, FormikErrors } from 'formik';

type InputTextAreaProps = {
  fieldName: string;
  label: string;

  handleChange: (field: string) => void;
};

const InputTextArea: React.FC<InputTextAreaProps> = ({
  fieldName,
  label,
  handleChange,
}) => {
  const [showPassword, setShowPassword] = useState(false);

  return (
    <div className="w-full">
      <p className="capitalize text-gray-500">{label}</p>

      <Field
        as={'textarea'}
        name={fieldName}
        onChange={handleChange}
        className="mt-[6px] flex flex-row min-w-full h-[200px] text-[14px] border px-4 py-2 rounded-md border-[#B4BAC6] focus:outline-none focus:border-primary focus:border-b-[1px]"
      />

      {/* {errors} */}
    </div>
  );
};

export default InputTextArea;
