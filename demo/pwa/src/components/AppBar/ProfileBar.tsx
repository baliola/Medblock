import React, { CSSProperties, FC } from 'react';

import Images from '@/constants/images';

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
  return (
    <div
      className="flex flex-row justify-between mx-6 items-center"
      style={style}
    >
      <div className="flex flex-row space-x-4 items-center">
        <img src={Images.dummyProfile} alt="" className="w-16" />
        <div className="flex flex-col items-start">
          <p className="text-gray-800 font-bold">I Putu Aryadi</p>
          <div className="flex flex-row items-start space-x-2 items-center">
            <img src={Images.male} alt="" className="w-3" />
            <p className="text-gray-800 font-bold text-sm">24 th</p>
            <p className="text-gray-800 text-sm">Maried</p>
          </div>
          <p className="text-gray-800 font-bold text-sm">
            EMR ID : 123123213213
          </p>
        </div>
      </div>

      <button onClick={onPressTrailing}>{trailingButton}</button>
    </div>
  );
};

export default ProfileBar;
