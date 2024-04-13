import React from 'react';
import {
  CheckCircle,
  CloseCircle,
  Filter,
  InfoCircle,
  Sort,
} from 'solar-icon-set/essentionalui';

import AppBar from '@/components/AppBar/AppBar';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const data = [
  {
    id: 1,
    icon: <CheckCircle color="green" size={18} iconStyle="Bold" />,
    title: 'Your EMR Has Been Access',
    subtitle: 'Pertamina Hospital-Nusantara City',
    dateTime: '24 Maret 2024 - 18.00 WIB',
    physician: 'Karyada Irawan',
  },
  {
    id: 2,
    icon: <InfoCircle color="blue" size={18} iconStyle="Bold" />,
    title: 'Your Medical Record Has Been Updated',
    subtitle: 'Pertamina Hospital-Nusantara City',
    dateTime: '22 Maret 2024 - 19.54 WIB',
    physician: 'Indra Kamataru',
  },
  {
    id: 3,
    icon: <CloseCircle color="red" size={18} iconStyle="Bold" />,
    title: 'Your EMR has been Revoked',
    subtitle: 'Sanglah Hospital-Denpasar',
    dateTime: '21 Maret 2024 - 00.34 WIB',
    physician: 'Samika Karamoy',
  },
];

const NotificationPage = () => {
  return (
    <Scaffold
      topBar={
        <div>
          <AppBar
            title={
              <p className="text-lg text-gray-800 font-bold">Notification</p>
            }
          />
          <div className="flex flex-row px-6 py-4 space-x-4 bg-white">
            <button
              className="flex flex-row items-center space-x-2"
              onClick={() => {}}
            >
              <Filter size={14} iconStyle="Bold" color="#242DA8" />
              <p>Filter</p>
            </button>
            <button
              className="flex flex-row items-center space-x-2"
              onClick={() => {}}
            >
              <Sort size={14} iconStyle="Bold" color="#242DA8" />
              <p>Filter</p>
            </button>
          </div>
        </div>
      }
    >
      <div className="p-6 mt-24">
        {data.map((item) => {
          return (
            <div
              key={item.id}
              className="w-full flex flex-row items-start mb-6 space-x-4"
            >
              {item.icon}
              <div>
                <p className="text-gray-800 font-bold">{item.title}</p>
                <p className="text-gray-800 text-xs my-1">{item.subtitle}</p>
                <p className="text-gray-800 text-xs">{item.dateTime}</p>
              </div>
            </div>
          );
        })}
      </div>
    </Scaffold>
  );
};

export default NotificationPage;
