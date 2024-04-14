import React, { Fragment } from 'react';
import { Combobox, Transition } from '@headlessui/react';

import { Field } from 'formik';
import { ChevronDownIcon, CheckIcon } from '@heroicons/react/20/solid';
import { DoctorType } from '@/interface/doctor.interface';

type Option = {
  id: string;
  name: string;

  // Add other properties as needed
};

type OptionFieldProps = {
  fieldName: string;
  options: DoctorType[] | undefined;
  query: string;
  values: any;
  setQuery: (value: string) => void;
  handleChange: (field: string) => void;
  setFieldValue: (field: string, value: any) => void;
};

const OptionField: React.FC<OptionFieldProps> = ({
  fieldName,
  options,
  query,
  values,
  handleChange,
  setFieldValue,
  setQuery,
}) => {
  const filter =
    query === ''
      ? options
      : options?.filter((option) => {
          return option.name.toLowerCase().includes(query.toLowerCase());
        });
  return (
    <div className="flex flex-col">
      <p className="capitalize text-gray-500">{fieldName}</p>
      <Combobox
        value={values.doctor}
        onChange={(value) => {
          console.log('fielde name', fieldName);
          setFieldValue(fieldName, value);
        }}
      >
        <div className="relative mt-1">
          <div className="relative w-full rounded-2xl cursor-default overflow-hidden border focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75 focus-visible:ring-offset-2 focus-visible:ring-offset-teal-300 sm:text-sm">
            <Combobox.Input
              className="flex w-full capitalize rounded-2xl border-none py-2 pl-3 pr-10 text-sm leading-5 text-gray-900 "
              displayValue={(doctor: any) => doctor.name}
              placeholder="Select admin doctor"
              id="doctor"
              name="doctor"
              onChange={(event) => {
                setQuery(event.target.value);
                handleChange(fieldName);
              }}
            />
            <Combobox.Button className="absolute inset-y-0 right-0 px-2 flex items-center bg-[#3E48D6]">
              <ChevronDownIcon
                className="h-5 w-5 text-gray-400"
                aria-hidden="true"
              />
            </Combobox.Button>
          </div>
          <Transition
            as={Fragment}
            leave="transition ease-in duration-100"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
            afterLeave={() => setQuery('')}
          >
            <Combobox.Options
              className="absolute mt-1 max-h-40 w-full overflow-auto rounded-2xl bg-white py-1 text-base shadow-lg ring-1 ring-black/5 focus:outline-none sm:text-sm"
              style={{ zIndex: 1 }}
            >
              {filter?.length === 0 && query !== '' ? (
                <div className="relative cursor-default select-none px-4 py-2 text-gray-700">
                  Nothing found.
                </div>
              ) : (
                filter?.map((option) => (
                  <Combobox.Option
                    key={option.id}
                    className={({ active }) =>
                      `relative cursor-default select-none py-2 pl-10 pr-4 ${
                        active ? 'bg-[#3E48D660] text-white' : 'text-gray-900'
                      }`
                    }
                    value={option}
                  >
                    {({ selected, active }) => (
                      <>
                        <span
                          className={`block truncate capitalize ${
                            selected ? 'font-medium' : 'font-normal'
                          }`}
                        >
                          {option.name}
                        </span>
                        {selected ? (
                          <span
                            className={`absolute inset-y-0 left-0 flex items-center pl-3 ${
                              active ? 'text-white' : 'text-teal-600'
                            }`}
                          >
                            <CheckIcon className="h-5 w-5" aria-hidden="true" />
                          </span>
                        ) : null}
                      </>
                    )}
                  </Combobox.Option>
                ))
              )}
            </Combobox.Options>
          </Transition>
        </div>
      </Combobox>
    </div>
  );
};

export default OptionField;
