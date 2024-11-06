"use client"

import { search } from "@/constants/contents/search";
import { Button, Icon, Input, InputGroup, InputLeftElement, InputRightElement } from "@chakra-ui/react";

interface SearchProps {
  defaultValue: string;
  onFillSearch: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onClearSearch: () => void;
}

export default function Search({
  defaultValue, onFillSearch, onClearSearch
}: SearchProps) {

  return (
    <InputGroup bg={'white'} rounded={'2xl'} size={'lg'}>
      <InputLeftElement pointerEvents='none'>
        <Icon as={search.leftIcon} color='#676767' mt={1} boxSize={5} />
      </InputLeftElement>
      <Input
        value={defaultValue}
        pr="4.5rem"
        onChange={onFillSearch}
        placeholder={search.placeholder}
        borderColor={'#A1A2A6'}
        rounded={'xl'}
        _placeholder={{ color: "#6F6F6F" }}
      />
      {defaultValue.length > 0 && (
        <InputRightElement w={'4.5rem'}>
          <Button h='1rem' size='xs' rounded={'full'} p={2} onClick={onClearSearch}>
            <Icon as={search.rightIcon} boxSize={2} />
          </Button>
        </InputRightElement>
      )}
    </InputGroup>
  )
}