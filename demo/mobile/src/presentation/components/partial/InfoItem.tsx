import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

interface InfoItemProps {
  icon: string;
  label?: string;
  data: string;
  subData?: string;
  classStyle?: string;
}

const InfoItem: React.FC<InfoItemProps> = ({
  data,
  icon,
  label,
  classStyle,
  subData,
}) => {
  return (
    <StyledView
      className={'flex flex-row items-center space-x-4 mb-4 ' + classStyle}>
      <GeneralImage url={icon} size={24} />
      <StyledView>
        {label ? (
          <TextPrimary text={label} classStyle="text-gray-800 text-xs mb-1" />
        ) : null}

        <TextPrimary text={data} classStyle="text-gray-800" isBold />
        {subData ? (
          <TextPrimary text={subData} classStyle="text-gray-800 text-xs mt-1" />
        ) : null}
      </StyledView>
    </StyledView>
  );
};

export default InfoItem;
