'use client';

import Modal from '@/components/Modal/Modal';
import Table from '@/components/Tables/Table';
import usePatientMock, { MockPatients } from '@/lib/MockData/mockData';
import { PlusIcon } from '@heroicons/react/20/solid';
import { ColumnDef } from '@tanstack/react-table';
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
import {
  Barcode,
  Health,
  ScanBarcode,
  SearchNormal1,
  User,
} from 'iconsax-react';
import { SetStateAction, useMemo, useState } from 'react';
import ModalAdd from './components/Modal/ModalAdd';
import { useRouter } from 'next/router';
import usePatient from '@/hooks/usePatient';
import { Patient } from 'declarations/patient_registry/patient_registry.did';

export default function DashboardExample() {
  // const { generateMockPatients } = usePatientMock();
  const [showModal, setShowModal] = useState<boolean>(false);
  const [showModalSuccess, setShowModalSuccess] = useState<boolean>(false);
  const router = useRouter();
  const { fetchPatient, patientList } = usePatient();

  const toggleModal = () => {
    setShowModal(!showModal);
  };

  const patientColumn = useMemo<ColumnDef<Patient>[]>(
    () => [
      {
        header: 'No',
        cell: (info) => <p className="font-normal w-auto">{info.row.index}</p>,
      },
      {
        header: 'Full Name',
        cell: (info) => (
          <p className="font-normal">{info.row.original.V1.address}</p>
        ),
      },
      {
        header: 'Date of Birth',
        cell: (info) => (
          <p className="font-normal">{info.row.original.V1.date_of_birth}</p>
        ), // Format the date as needed
      },
      {
        header: 'Place of Birth',
        cell: (info) => (
          <p className="font-normal"> {info.row.original.V1.place_of_birth}</p>
        ),
      },
      {
        header: 'Address',
        cell: (info) => (
          <p className="font-normal">{info.row.original.V1.address}</p>
        ),
      },
      {
        header: 'Action',
        cell: (info) => (
          <div className="flex gap-2">
            <ScanBarcode
              size="24"
              color="#FDB569"
              className="cursor-pointer"
              onClick={toggleModal}
            />
            <Health
              size="24"
              color="#3E48D6"
              className="cursor-pointer"
              onClick={() => {
                router.push(`/emr/1`);
              }}
            />
            <User size="24" className="cursor-pointer" />
          </div>
        ),
      },
    ],
    [],
  );

  return (
    <>
      <div className="flex flex-col p-4 md:p-6 gap-4">
        {/* HEADER */}
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
            <button
              className="flex  items-center border-[2px] p-2 w-auto outline-hover justify-center align-middle  bg-[#242DA8] transition-all ease-in duration-200 text-white rounded-2xl  border-none text-[14px] font-normal hover:bg-opacity-40"
              onClick={toggleModal}
            >
              {/* <img src={} alt="" /> */}
              <PlusIcon width={16} />
              Add Patient
            </button>
          </div>
        </div>
        {/* Content */}
        <Card className="flex flex-col gap-2 mt-4">
          <Table
            columns={patientColumn}
            data={patientList ?? []}
            isLoading={false}
            currentPage={0}
            // setCurrentPage={setCurrentPageAccountType}
            totalPage={10}
            limitPage={10}
            isCommon={true}
          />
        </Card>
      </div>
      <Modal toggle={toggleModal} isOpen={showModal}>
        <ModalAdd
          setShowModalPhone={setShowModal}
          setShowModalSuccess={setShowModalSuccess}
          // setPhoneNumber={undefined}
          datas={''}
        />
      </Modal>
    </>
  );
}
