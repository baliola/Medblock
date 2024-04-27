import React, { useEffect, useState } from 'react';
import {
  CheckCircle,
  CloseCircle,
  Filter,
  InfoCircle,
  Sort,
} from 'solar-icon-set';

import AppBar from '@/components/AppBar/AppBar';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';
import usePatient from '@/hooks/usePatient';
import { useAuth } from '@/config/agent';
import { ActivityType } from 'declarations/patient_registry/patient_registry.did';
import useProvider from '@/hooks/useProvider';

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

// const getActivityText = (activity: any) => {
//   switch (activity) {
//     case 'Updated':
//       return 'Your Medical Record Has Been Updated';
//     case 'Accessed':
//       return 'Your EMR Has Been Accessed';
//     case 'Revoked':
//       return 'Your EMR Has Been Revoked';
//     default:
//       return 'Unknown Activity';
//   }
// };

const NotificationPage = () => {
  const { identity } = useAuth();
  const {
    getNotifications,
    notifications,
    getActivityText,
    findUpdatedNotification,
  } = usePatient();

  const { GetProviderInfo } = useProvider();

  const [providerNames, setProviderNames] = useState<{
    [key: string]: string;
  }>({});

  useEffect(() => {
    if (identity) getNotifications();
  }, [identity]);

  useEffect(() => {
    if (notifications) {
      const fetchProviderNames = async () => {
        const names: { [key: string]: string } = {};
        for (const notification of notifications) {
          const name = await GetProviderInfo(notification.provider_id);
          names[notification.provider_id] = name || '';
        }
        setProviderNames(names);
      };
      fetchProviderNames();
    }
  }, [notifications, GetProviderInfo]);

  function getIconForActivity(activity: ActivityType) {
    switch (Object.keys(activity)[0]) {
      case 'Accessed':
        return <CheckCircle color="green" size={18} iconStyle="Bold" />;
      case 'Updated':
        return <InfoCircle color="blue" size={18} iconStyle="Bold" />;
      case 'Revoked':
        return <CloseCircle color="red" size={18} iconStyle="Bold" />;
      default:
        return null; // Handle other cases if needed
    }
  }

  const formatDateFromBigIntWithTime = (timestamp: bigint): string => {
    // Convert the BigInt to a number
    const second = Number(timestamp);

    // Convert nanoseconds to milliseconds
    const timeInSeconds = Math.floor(second / 1e6);

    // Create a Date object from the milliseconds
    const date = new Date(timeInSeconds);

    // Get day, month, and year
    const day = ('0' + date.getDate()).slice(-2);

    // Array of month names
    const monthNames = [
      'Januari',
      'Februari',
      'Maret',
      'April',
      'Mei',
      'Juni',
      'Juli',
      'Agustus',
      'September',
      'Oktober',
      'November',
      'Desember',
    ];

    const monthIndex = date.getMonth();
    const monthName = monthNames[monthIndex];

    const year = date.getFullYear();

    // Get hours and minutes
    const hours = ('0' + date.getHours()).slice(-2);
    const minutes = ('0' + date.getMinutes()).slice(-2);

    // Format the date as dd/mm/yyyy hh:mm
    const formattedDate = `${day} ${monthName} ${year} - ${hours}.${minutes}`;

    return formattedDate;
  };

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
        {notifications &&
          notifications.map((item, index) => (
            <div
              key={index}
              className="w-full flex flex-row items-start mb-6 space-x-4"
            >
              {getIconForActivity(item.activity_type)}{' '}
              {/* Use getIconForActivity function */}
              <div>
                <p className="text-gray-800 font-bold">
                  {getActivityText(item.activity_type)}
                </p>
                <p className="text-gray-800 text-xs my-1 capitalize">
                  {providerNames[item.provider_id] ?? ''}
                </p>
                <p className="text-gray-800 text-xs">
                  {formatDateFromBigIntWithTime(item.timestamp)}
                </p>
              </div>
            </div>
          ))}
        {/* {data.map((item) => {
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
        })} */}
      </div>
    </Scaffold>
  );
};

export default NotificationPage;
