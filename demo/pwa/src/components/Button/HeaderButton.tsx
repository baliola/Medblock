import React from 'react';
import { ArrowRight } from 'solar-icon-set/arrows';

interface HeaderButtonProps {
  icon: string;
  label: string;
  onPress: () => void;
  classStyle?: string;
}

const HeaderButton: React.FC<HeaderButtonProps> = ({
  icon,
  label,
  onPress,
  classStyle,
}) => {
  return (
    <div
      onClick={() => {}}
      className={'flex flex-row justify-between items-center ' + classStyle}
    >
      <div className="flex flex-row space-x-2">
        <img src={icon} alt="" className="w-8" />
        <p className="text-gray-800 text-xl">{label}</p>
      </div>
      <ArrowRight color="black" iconStyle="Bold" size={18} />
    </div>
  );
};

export default HeaderButton;
