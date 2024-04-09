import TextPrimary from '@components/text/TextPrimary';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

interface MetaItemProps {
  label: string;
  data: string;
}

const MetaItem: React.FC<MetaItemProps> = ({data, label}) => {
  return (
    <StyledView className="flex flex-col">
      <TextPrimary text={label} classStyle="text-gray-800 text-xs" />
      <TextPrimary text={data} classStyle="text-gray-800 text-lg" isBold />
    </StyledView>
  );
};

export default MetaItem;
