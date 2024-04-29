import React, { ChangeEvent } from 'react';
import { ArrowDown } from 'solar-icon-set/arrows';

interface Props {
  value: string;
  onChange: (e: ChangeEvent<HTMLSelectElement>) => void;
  label: string;
  options: string[];
  classStyle?: string;
  name?: string;
  id?: string;
  placeholder?: string;
}

const InputDropdown: React.FC<Props> = ({
  value,
  onChange,
  label,
  options,
  classStyle,
  name,
  id,
  placeholder,
}) => {
  return (
    <div className="w-full">
      <label
        htmlFor="name"
        className="block text-sm font-medium leading-6 text-gray-900 mb-2"
      >
        {label}
      </label>
      <select
        id={id}
        name={name}
        placeholder={placeholder}
        // value={value}
        onChange={onChange}
        className={
          'block w-full rounded-xl border-0 px-4 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-[#242DA8] sm:text-sm sm:leading-6 appearance-none ' +
          classStyle
        }
      >
        <option value="">{placeholder || 'Select Option'}</option>{' '}
        {/* Default option */}
        {options.map((option, index) => (
          <option key={index} value={option}>
            {option}
          </option>
        ))}
      </select>
    </div>
  );
};

export default InputDropdown;
