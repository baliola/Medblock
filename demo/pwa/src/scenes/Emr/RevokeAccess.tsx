'use client';
import { useRouter } from 'next/router';
import React, { useState } from 'react';
import { Magnifer } from 'solar-icon-set/search';

import AppBarWithIcon from '@/components/AppBar/AppBarWithIcon';
import PrimaryButton from '@/components/Button/PrimaryButton';
import DialogBasic from '@/components/Dialog/DialogBasic';
import InputText from '@/components/input/InputText';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const data = [
  {
    id: 1,
    selected: true,
    title: 'Sanglah Hospital-Denpasar',
    latest: '24 Maret 2024',
    physician: 'Karyada Irawan',
  },
  {
    id: 2,
    selected: false,
    title: 'Pertamina',
    latest: '22 Maret 2024',
    physician: 'Indra Kamataru',
  },
  {
    id: 3,
    selected: false,
    title: 'DR. Soedjono Hospital',
    latest: '21 Maret 2024',
    physician: 'Samika Karamoy',
  },
];

const RevokeAccess = () => {
  const router = useRouter();
  const [open, setOpen] = useState<boolean>(false);
  const [openError, setOpenError] = useState<boolean>(false);

  const handleRevoke = () => {
    setOpen(false);
    setOpenError(true);
  };

  return (
    <Scaffold
      topBar={
        <AppBarWithIcon
          title={
            <p>
              Find and select hospital that you want to revoke your EMR access
            </p>
          }
          child={
            <div className="px-6">
              <InputText
                value=""
                classStyle="px-6 mt-4"
                placeholder={'Search'}
              />
            </div>
          }
        />
      }
      bottomChild={
        <div className="p-6">
          <PrimaryButton
            title={'Revoke my EMR'}
            onSubmit={() => {
              setOpen(true);
            }}
          />
        </div>
      }
    >
      <>
        <div className="p-6 mt-52">
          {data.map((item) => (
            <div
              key={item.id}
              className="w-full flex flex-row items-center mb-6 space-x-4"
            >
              <input
                id={item.id}
                aria-describedby="comments-description"
                name={item.title}
                type="checkbox"
                className="h-4 w-4 rounded border-gray-300 text-primary-normal focus:ring-primary-normal"
              />
              <img src={Images.hospital} alt="" className="w-16" />
              <div>
                <p className="text-gray-800 font-bold"> {item.title} </p>
                <p className="text-gray-800 text-xs my-1">
                  Last Visited: {item.latest}{' '}
                </p>
                <p className="text-gray-800 text-xs">
                  Physician: {item.physician}{' '}
                </p>
              </div>
            </div>
          ))}
        </div>
        <DialogBasic
          open={open}
          onCancel={() => {
            setOpen(false);
          }}
          onYes={() => {
            handleRevoke();
          }}
          title="Alert"
          labelCancel="Cancel"
          labelYes="Revoke"
          child={
            <p className="text-sm text-gray-500">
              Close EMR Access to selected hospital?
            </p>
          }
        />
        <DialogBasic
          open={openError}
          onCancel={() => {
            setOpenError(false);
            router.back();
          }}
          labelCancel="Close"
          child={
            <div className="flex flex-col justify-center items-center">
              <img src={Images.error} alt="" className="w-32" />
              <p className="text-sm text-gray-500 font-bold mt-2">
                Your EMR access to selected hospital is closed.
              </p>
            </div>
          }
        />
      </>
    </Scaffold>
  );
};

export default RevokeAccess;
