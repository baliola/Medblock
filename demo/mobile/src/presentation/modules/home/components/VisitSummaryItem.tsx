import TextPrimary from '@components/text/TextPrimary';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

interface VisitSummaryItemProps {
  label: string;
  data: string;
}

const VisitSummaryItem: React.FC<VisitSummaryItemProps> = ({data, label}) => {
  return (
    <StyledView className="mt-4">
      <TextPrimary text={label} classStyle="text-gray-400 text-xs" />
      <TextPrimary text={data} classStyle="text-gray-800 mt-2" />
    </StyledView>
  );
};

export default VisitSummaryItem;
