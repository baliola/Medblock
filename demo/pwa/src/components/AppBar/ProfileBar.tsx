import React, { CSSProperties, FC } from 'react';

import Images from '@/constants/images';
import useEMRPatient from '@/hooks/useEmrPatient';

interface ProfileBarProps {
  style?: CSSProperties;
  trailingButton: React.ReactElement<any, any>;
  onPressTrailing: () => void;
}

const ProfileBar: FC<ProfileBarProps> = ({
  onPressTrailing,
  trailingButton,
  style,
}) => {
  const { patientInfo } = useEMRPatient();
  function calculateAge(dateOfBirth: string): number {
    const today: Date = new Date();
    const birthDate: Date = new Date(dateOfBirth);

    let age: number = today.getFullYear() - birthDate.getFullYear();
    const monthDifference: number = today.getMonth() - birthDate.getMonth();

    // If the current month is before the birth month or
    // if the current month is the same as the birth month but the current day is before the birth day,
    // subtract 1 from the age
    if (
      monthDifference < 0 ||
      (monthDifference === 0 && today.getDate() < birthDate.getDate())
    ) {
      age--;
    }

    return age;
  }
  return (
    <div
      className="flex flex-row justify-between mx-6 items-center mt-4"
      style={style}
    >
      <div className="flex flex-row space-x-4 items-center">
        <img src={Images.dummyProfile} alt="" className="w-16" />
        <div className="flex flex-col items-start">
          <p className="text-gray-800 font-bold">
            {patientInfo ? patientInfo.V1.name : ''}
          </p>
          <div className="flex flex-row items-start space-x-2 items-center">
            <img src={Images.male} alt="" className="w-3" />
            <p className="text-gray-800 font-bold text-sm">
              {' '}
              {patientInfo ? calculateAge(patientInfo.V1.date_of_birth) : ''}
            </p>
            <p className="text-gray-800 text-sm">
              {' '}
              {patientInfo ? patientInfo.V1.martial_status : ''}
            </p>
          </div>
          <p className="text-gray-800 font-bold text-sm">EMR ID : -</p>
        </div>
      </div>

      <button onClick={onPressTrailing}>{trailingButton}</button>
    </div>
  );
};

export default ProfileBar;
