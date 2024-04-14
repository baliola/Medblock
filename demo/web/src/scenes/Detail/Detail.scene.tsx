import { NextPageWithLayout } from '@/types';
import React, { useMemo, useState } from 'react';
import PatientInfo from './component/PatientInfo';
import Link from 'next/link';
import { ColumnDef } from '@tanstack/react-table';
import usePatientMock, { MockPatients } from '@/lib/MockData/mockData';
import { Card, DatePicker } from '@tremor/react';
import Table from '@/components/Tables/Table';
import useMedicalRecordMock, {
  MockMedicalRecord,
} from '@/lib/MockData/MockPatientEMR';
import dateFormatter from '@/lib/dateFormatter';
import { Health, SearchNormal1 } from 'iconsax-react';
import { PlusIcon } from '@heroicons/react/20/solid';
import { useRouter } from 'next/router';
// import Datepicker from 'react-tailwindcss-datepicker';

const DetailPatient: NextPageWithLayout = () => {
  // const { generateMockMedicalRecords } = useMedicalRecordMock();
  const router = useRouter();

  const [dateValue, setDateValue] = useState({
    startDate: new Date(),
    endDate: new Date(),
  });

  const handleChangeDate = (newValue: any) => {
    console.log('newValue:', newValue);
    setDateValue(newValue);
  };

  const patientColumn = useMemo<ColumnDef<MockMedicalRecord>[]>(
    () => [
      {
        header: 'No',
        cell: (info) => <p className="font-normal w-auto">{info.row.index}</p>,
      },
      {
        header: 'Date',
        cell: (info) => (
          <p className="font-normal">
            {dateFormatter(info.row.original.createdAt)}
          </p>
        ), // Format the date as needed
      },
      {
        header: 'Hospital',
        cell: (info) => (
          <p className="font-normal">{info.row.original.hospitalName}</p>
        ),
      },
      {
        header: 'Physician',
        cell: (info) => (
          <p className="font-normal">{info.row.original.doctorName}</p>
        ),
      },
      {
        header: 'Last update',
        cell: (info) => (
          <p className="font-normal">{info.row.original.diagnosis}</p>
        ),
      },
      {
        header: 'Action',
        cell: (info) => (
          <div className="flex gap-2">
            <Health
              size="24"
              color="#3E48D6"
              className="cursor-pointer"
              onClick={() => {
                router.push(`/medical-record/${info.row.original.id}`);
              }}
            />
          </div>
        ),
      },
    ],
    [],
  );
  return (
    // <div className="grid md:grid-cols-[240px_1fr] w-screen overflow-x-hidden">
    <div className="flex p-4">
      <div className="w-full overflow-x-auto max-w-[1440px] mx-auto">
        {/* <Navbar isOpen={isSidebarOpen} sidebarChange={toggleSidebar} /> */}
        <div className="flex flex-col pl-24 pr-2 gap-4">
          <div className="flex flex-col w-full gap-4">
            <div className="flex flex-col gap-1">
              <p className="text-xl font-medium text-gray-800 w-auto">
                Medical Record List
              </p>{' '}
              <div className="flex text-sm font-medium  gap-2">
                <Link href="/" className="text-[#06B8EE] underline">
                  Patient Management
                </Link>{' '}
                <span> &gt; Patient EMR</span>
              </div>{' '}
            </div>
            <div className="flex w-full justify-end">
              <button
                className="flex  items-center border-[2px] p-2 w-auto outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40"
                onClick={() => {
                  router.push(`/medical-record/add`);
                }}
              >
                {/* <img src={} alt="" /> */}
                <PlusIcon width={16} />
                Add new EMR
              </button>
            </div>
            <div className="flex w-full justify-end gap-4">
              <div className="flex">
                {/* <DatePicker /> */}

                <div className="relative max-w-sm">
                  <input
                    type="date"
                    className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-2xl focus:ring-blue-500 focus:border-blue-500 block w-full  p-2 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 "
                    placeholder="Select date"
                  />
                </div>

                {/* <Datepicker
                    popoverDirection="up"
                    primaryColor={'purple'}
                    minDate={new Date()}
                    // maxDate={data?.end_time && data?.end_time}
                    displayFormat={'MMMM DD YYYY'}
                    placeholder={'Filter by date'}
                    value={dateValue}
                    separator="-"
                    onChange={handleChangeDate}
                    inputClassName="relative  w-full rounded-md border border-secondary text-black font-normal bg-white p-2 z-[9999]"
                  />{' '} */}
              </div>
              <div className="flex ">
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
            </div>
            <Card className="flex flex-col gap-2 mt-4">
              <Table
                columns={patientColumn}
                data={[]}
                isLoading={false}
                currentPage={0}
                // setCurrentPage={setCurrentPageAccountType}
                totalPage={1}
                limitPage={100}
                isCommon={true}
              />
            </Card>
          </div>
        </div>{' '}
      </div>
    </div>
    // </div>
  );
};

export default DetailPatient;
DetailPatient.patientLayout = true;
