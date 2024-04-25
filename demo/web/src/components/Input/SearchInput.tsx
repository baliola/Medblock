import React, { Fragment } from 'react';
import { Combobox, Transition } from '@headlessui/react';

import { Field } from 'formik';
import { ChevronDownIcon, CheckIcon } from '@heroicons/react/20/solid';
import { DoctorType } from '@/interface/doctor.interface';
import { SearchNormal1 } from 'iconsax-react';
import { PatientWithNikAndSession } from 'declarations/patient_registry/patient_registry.did';
import { useCentralStore } from '@/Store';
import { useRouter } from 'next/router';

type Option = {
  id: string;
  name: string;

  // Add other properties as needed
};

type SearchComponentProps = {
  fieldName: string;
  options: PatientWithNikAndSession | null;
  query: string;
  // values: any;
  setQuery: (value: string) => void;
  handleChange: (field: string) => void;
  //   setFieldValue: (field: string, value: any) => void;
};

const SearchComponent: React.FC<SearchComponentProps> = ({
  fieldName,
  options,
  query,
  //   values,
  handleChange,
  //   setFieldValue,
  setQuery,
}) => {
  const { isLoading } = useCentralStore();
  const router = useRouter();
  //   const filter: PatientWithNikAndSession =
  //     query === ''
  //       ? options
  //       : options?.info.V1.name.toLowerCase().includes(query.toLowerCase());

  console.log('loading', isLoading);

  return (
    <div className="flex flex-col w-full">
      <Combobox>
        <div className="relative mt-1 w-full">
          <div className="relative w-full rounded-2xl cursor-default overflow-hidden border focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75 focus-visible:ring-offset-2 focus-visible:ring-offset-teal-300 sm:text-sm">
            <Combobox.Input
              className="flex w-full capitalize rounded-2xl border-none py-2 pl-3 pr-10 text-sm leading-5 text-gray-900 "
              displayValue={(doctor: any) => doctor.name}
              placeholder="Search Patient"
              id="doctor"
              name="doctor"
              onChange={(event) => {
                if (event.target.value.length === 16) {
                  setQuery(event.target.value);
                  handleChange(event.target.value);
                }
              }}
            />
            <Combobox.Button className="absolute inset-y-0 right-0 px-2 flex items-center bg-[#3E48D6]">
              <SearchNormal1
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
              {!isLoading && (
                <>
                  {options ? (
                    <Combobox.Option
                      className={({ active }) =>
                        `relative cursor-default select-none py-2 pl-10 pr-4 ${
                          active ? 'bg-[#3E48D660] text-white' : 'text-gray-900'
                        }`
                      }
                      value={options}
                    >
                      {({ selected, active }) => (
                        <>
                          <span
                            className={`block truncate capitalize ${
                              selected ? 'font-medium' : 'font-normal'
                            }`}
                            onClick={() => {
                              router.push(`/emr/${options.session_id}`);
                            }}
                          >
                            {options.info.V1.name}
                          </span>
                          {selected ? (
                            <span
                              className={`absolute inset-y-0 left-0 flex items-center pl-3 ${
                                active ? 'text-white' : 'text-teal-600'
                              }`}
                            >
                              <CheckIcon
                                className="h-5 w-5"
                                aria-hidden="true"
                              />
                            </span>
                          ) : null}
                        </>
                      )}
                    </Combobox.Option>
                  ) : (
                    <div className="relative cursor-default select-none px-4 py-2 text-gray-700">
                      Nothing found.
                    </div>
                  )}
                </>
              )}
            </Combobox.Options>
          </Transition>
        </div>
      </Combobox>
    </div>
  );
};

export default SearchComponent;
