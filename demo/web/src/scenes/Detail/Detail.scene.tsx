import { NextPageWithLayout } from '@/types';
import React, { useEffect, useMemo, useState } from 'react';
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
import useEMRPatient from '@/hooks/useEmrPatient';
import { useCentralStore } from '@/Store';
import {
  EmrHeader,
  EmrHeaderWithStatus,
} from 'declarations/patient_registry/patient_registry.did';
import { formatDateFromBigInt } from '@/lib/bigintDateFormat';
import { useAuth } from '@/config/agent';
// import Datepicker from 'react-tailwindcss-datepicker';
export type DetailType = {
  name: string;
  sessionId: string;
};

const DetailPatient = (props: DetailType) => {
  const { identity, authenticated } = useAuth();

  const { name, sessionId } = props;

  // const { generateMockMedicalRecords } = useMedicalRecordMock();
  const router = useRouter();
  const { getPatientInfo, emrList, isLoading, GetEmr } = useEMRPatient();
  const { nik } = useCentralStore();

  const [selectedDate, setSelectedDate] = useState<Date | null>(null);

  const [searchQuery, setSearchQuery] = useState('');
  console.log('session ui', sessionId);

  const formatDateToString = (date: Date | null): string => {
    if (!date) return ''; // If date is null, return an empty string

    const day = ('0' + date.getDate()).slice(-2);
    const month = ('0' + (date.getMonth() + 1)).slice(-2);
    const year = date.getFullYear();

    return `${day}/${month}/${year}`;
  };

  const filteredEmrList = useMemo(() => {
    return emrList.filter((emr) => {
      const isMatchHospitalName = emr.hospital_name
        .toLowerCase()
        .includes(searchQuery.toLowerCase());

      const isMatchDate =
        !selectedDate ||
        formatDateFromBigInt(emr.status.created_at) ===
          formatDateToString(selectedDate);
      console.log('ismacth date', formatDateFromBigInt(emr.status.created_at));
      console.log('ismacth name', formatDateToString(selectedDate));

      return isMatchHospitalName && isMatchDate;
    });
  }, [emrList, searchQuery, selectedDate]);

  const handleChangeSearch = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchQuery(event.target.value);
  };

  const handleChangeDates = (date: Date | null) => {
    console.log('ismacth date change', date);
    setSelectedDate(date);
  };

  useEffect(() => {
    if (identity) {
      // getPatientInfo(sessionId, name);
      GetEmr(sessionId);
    }
  }, [identity]);

  const patientColumn = useMemo<ColumnDef<EmrHeaderWithStatus>[]>(
    () => [
      {
        header: 'EMR ID',
        cell: (info) => (
          <p className="font-normal">{info.row.original.header.emr_id}</p>
        ), // Format the date as needed
      },
      {
        header: 'Hospital name',
        cell: (info) => (
          <p className="font-normal">{info.row.original.hospital_name}</p>
        ),
      },

      {
        header: 'Issued At',
        cell: (info) => (
          <p className="font-normal">
            {formatDateFromBigInt(info.row.original.status.created_at)}
          </p>
        ),
      },

      {
        header: 'Action',
        cell: (info) => (
          <>
            {isLoading ? (
              <p>Loading...</p>
            ) : (
              <div className="flex gap-2">
                <Health
                  size="24"
                  color="#3E48D6"
                  className="cursor-pointer"
                  onClick={() => {
                    router.push({
                      pathname: `/medical-record/edit/${info.row.original.header.emr_id}`,
                      query: {
                        providerId: info.row.original.header.provider_id,
                        sessions: sessionId as string,
                      },
                    });
                  }}
                />
              </div>
            )}
          </>
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
                  router.push({
                    pathname: `/medical-record/add/${nik}`,
                    query: {
                      sessions: sessionId as string,
                    },
                  });
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
                    value={
                      selectedDate
                        ? selectedDate.toISOString().split('T')[0]
                        : ''
                    }
                    onChange={(e) => handleChangeDates(e.target.valueAsDate)}
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
                    value={searchQuery}
                    onChange={handleChangeSearch}
                    required
                  />
                </div>
              </div>
            </div>
            <Card className="flex flex-col gap-2 mt-4">
              <Table
                columns={patientColumn}
                data={filteredEmrList}
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
