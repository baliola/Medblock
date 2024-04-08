'use client';

import { PlusIcon } from '@heroicons/react/20/solid';
import {
  Card,
  Grid,
  Tab,
  TabGroup,
  TabList,
  TabPanel,
  TabPanels,
  Text,
  Title,
} from '@tremor/react';
import { SearchNormal1 } from 'iconsax-react';

export default function DashboardExample() {
  return (
    <div className="flex flex-col p-4 md:p-6 gap-4">
      <p className="text-xl font-medium text-gray-800 w-auto">
        Medblock Patient Management
      </p>
      <div className="flex flex-row justify-between">
        <div className="flex w-full">
          <label className="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">
            Search
          </label>
          <div className="relative w-full">
            <div className="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
              <SearchNormal1 size={16} />
            </div>
            <input
              type="search"
              id="default-search"
              className="flex w-full max-w-[300px] p-2 ps-10 text-sm text-gray-900 border border-gray-300 rounded-2xl bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              placeholder="Search patient"
              required
            />
          </div>
        </div>
        <div className="flex w-full justify-end">
          <button className="flex  items-center border-[2px] p-2 w-auto outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40">
            {/* <img src={} alt="" /> */}
            <PlusIcon width={16} />
            Add Patient
          </button>
        </div>
      </div>
    </div>
  );
}
