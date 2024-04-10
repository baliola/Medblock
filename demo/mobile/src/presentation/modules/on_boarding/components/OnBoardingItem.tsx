import {styled} from 'nativewind';
import React from 'react';
import {Dimensions, Image, StyleSheet, View} from 'react-native';
import {OnBoardingItemData} from '../state/OnBoardingData';
import TextPrimary from '@components/text/TextPrimary';

const StyledView = styled(View);
const StyledImage = styled(Image);

export const SLIDER_WIDTH = Dimensions.get('window').width;
export const ITEM_WIDTH = Math.round(SLIDER_WIDTH);

interface OnBoardingItemProps {
  item: OnBoardingItemData;
}

const OnBoardingItem: React.FC<OnBoardingItemProps> = ({item}) => {
  return (
    <StyledView className="flex flex-col justify-center items-center px-10">
      <TextPrimary
        text={item.title}
        classStyle="text-2xl text-center mb-8 text-gray-800"
        isBold={true}
      />
      <StyledView style={styles.containerIcon} className="mb-6">
        <StyledImage
          source={parseInt(item.icon, 10)}
          className="h-full w-full"
          style={styles.icon}
        />
      </StyledView>
      <TextPrimary
        text={item.body}
        classStyle="text-center text-gray-800 text-xs"
      />
    </StyledView>
  );
};

const styles = StyleSheet.create({
  containerIcon: {width: ITEM_WIDTH * 0.7, height: ITEM_WIDTH * 0.6},
  icon: {objectFit: 'scale-down'},
});

export default OnBoardingItem;
