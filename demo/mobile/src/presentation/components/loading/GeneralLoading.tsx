import React from 'react';
import Images from '@constants/images';
import {styled} from 'nativewind';
import {Image, View} from 'react-native';
import Dialog from 'react-native-dialog';
import Colors from '@constants/colors';
const StyledView = styled(View);
const StyledImage = styled(Image);

interface GeneralLoadingProps {
  loading: boolean;
}

const GeneralLoading: React.FC<GeneralLoadingProps> = ({loading}) => {
  return (
    <Dialog.Container
      visible={loading}
      contentStyle={{
        borderRadius: 30,
        elevation: 0,
        backgroundColor: Colors.white,
      }}
      headerStyle={{display: 'none'}}>
      <StyledView className="bg-white h-40 w-full rounded-xl flex justify-center items-center">
        <StyledView className="h-20 w-20 mr-2">
          <StyledImage
            source={parseInt(Images.logo, 10)}
            className="h-full w-full"
            style={{objectFit: 'scale-down'}}
          />
        </StyledView>
      </StyledView>
    </Dialog.Container>
  );
};

export default GeneralLoading;
