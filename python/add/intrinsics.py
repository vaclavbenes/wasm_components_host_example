
def _clamp(i: int, min: int, max: int) -> int:
                        if i < min or i > max:
                            raise OverflowError(f'must be between {min} and {max}')
                        return i
                
